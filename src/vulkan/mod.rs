use std::collections::HashMap;
use std::ptr;
use std::rc::Rc;

use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;

use winit::window::Window;

mod command_pool;
pub mod constants;
mod descriptor_pool;
mod descriptor_set_layout;
mod dynamic_texture;
mod framebuffers;
mod image_views;
mod instance;
mod logical_device;
mod physical_device;
mod pipeline;
mod platform;
mod queue_indices;
mod queues;
mod render_pass;
pub mod sampler;
mod surface;
mod swap_chain;
mod swap_chain_support;
mod sync_objects;
pub mod texture;
mod uniform_buffers;
mod uniform_layout;
mod util;
pub mod window;

use command_pool::CommandPool;
use descriptor_pool::DescriptorPool;
use descriptor_set_layout::DescriptorSetLayout;
use framebuffers::Framebuffers;
use image_views::ImageViews;
use instance::Instance;
use logical_device::LogicalDevice;
use physical_device::PhysicalDevice;
use pipeline::Pipeline;
use queues::Queues;
use render_pass::RenderPass;
use sampler::Sampler;
use surface::Surface;
use swap_chain::SwapChain;
use sync_objects::SyncObjects;
use texture::{DynamicTexture, StaticTexture};
use uniform_buffers::UniformBuffers;
use uniform_layout::{UniformLayout, UniformLayouts};

use crate::volume::Volume;

pub struct VulkanBuilder {
    _entry: ash::Entry,
    window: Rc<Window>,
    instance: Instance,
    surface: Surface,
    physical_device: PhysicalDevice,
    logical_device: LogicalDevice,
    swap_chain: SwapChain,
    image_views: ImageViews,
    queues: Queues,
    render_pass: RenderPass,
    framebuffers: Framebuffers,
    command_pool: CommandPool,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    uniforms: UniformLayouts<UniformBuffers>,
    textures: UniformLayouts<StaticTexture>,
    dynamic_textures: UniformLayouts<DynamicTexture>,
}

impl VulkanBuilder {
    fn new(window: Rc<Window>) -> Self {
        let entry = unsafe { ash::Entry::new() }.unwrap();

        debug_assert!(
            !constants::ENABLE_VALIDATION || Self::check_validation_layers_support(&entry),
            "Validation layers not supported"
        );

        let instance = Instance::new(&entry);
        let surface = Surface::new(&entry, &instance, window.as_ref());
        let physical_device = PhysicalDevice::new(&instance, &surface);
        let logical_device = LogicalDevice::new(&instance, &physical_device);
        let swap_chain = SwapChain::new(
            &instance,
            &surface,
            &physical_device,
            &logical_device,
            window.as_ref(),
        );
        let image_views = ImageViews::new(&logical_device, &swap_chain);
        let queues = Queues::new(&logical_device, &physical_device.indices);
        let render_pass = RenderPass::new(&logical_device, &swap_chain);
        let framebuffers =
            Framebuffers::new(&logical_device, &swap_chain, &image_views, &render_pass);
        let command_pool = CommandPool::new(&logical_device, &physical_device.indices);
        let memory_properties = unsafe {
            instance
                .value
                .get_physical_device_memory_properties(physical_device.value)
        };

        VulkanBuilder {
            _entry: entry,
            window,
            instance,
            surface,
            physical_device,
            logical_device,
            swap_chain,
            image_views,
            queues,
            render_pass,
            framebuffers,
            command_pool,
            memory_properties,
            uniforms: HashMap::new(),
            textures: HashMap::new(),
            dynamic_textures: HashMap::new(),
        }
    }

    pub fn with_uniform<T>(mut self, binding: u32, stage_flags: vk::ShaderStageFlags) -> Self {
        let uniform_buffers = UniformBuffers::new(
            self.memory_properties,
            &self.logical_device,
            self.swap_chain.images.len(),
            std::mem::size_of::<T>() as vk::DeviceSize,
        );

        self.uniforms.insert(
            binding,
            UniformLayout {
                stage_flags,
                uniforms: uniform_buffers,
            },
        );
        self
    }

    pub fn with_texture(
        mut self,
        binding: u32,
        stage_flags: vk::ShaderStageFlags,
        volume: &Volume,
    ) -> Self {
        let texture = StaticTexture::new_3d(
            self.memory_properties,
            &self.logical_device,
            &self.command_pool,
            &self.queues,
            volume.size() as u32,
            volume.size() as u32,
            volume.size() as u32,
            2,
            vk::Format::R16_UINT,
            &volume.data,
        );

        self.textures.insert(
            binding,
            UniformLayout {
                stage_flags,
                uniforms: texture,
            },
        );
        self
    }

    pub fn with_dynamic_texture(
        mut self,
        binding: u32,
        stage_flags: vk::ShaderStageFlags,
        volume: &Volume,
    ) -> Self {
        let texture = DynamicTexture::new_3d(
            self.memory_properties,
            &self.logical_device,
            &self.command_pool,
            &self.queues,
            volume.size() as u32,
            volume.size() as u32,
            volume.size() as u32,
            2,
            vk::Format::R16_UINT,
        );

        self.dynamic_textures.insert(
            binding,
            UniformLayout {
                stage_flags,
                uniforms: texture,
            },
        );
        self
    }

    pub fn build(mut self) -> Vulkan {
        let sampler = Sampler::new(&self.logical_device);
        let descriptor_set_layout = DescriptorSetLayout::new(
            &self.logical_device,
            &self.uniforms,
            &self.textures,
            &self.dynamic_textures,
        );
        let pipeline = Pipeline::new(
            &self.logical_device,
            &self.swap_chain,
            &self.render_pass,
            &descriptor_set_layout,
        );
        let descriptor_pool = DescriptorPool::new(
            &self.logical_device,
            self.swap_chain.images.len(),
            &descriptor_set_layout,
            &self.uniforms,
            &self.textures,
            &self.dynamic_textures,
            &sampler,
        );
        self.command_pool.create_buffers(
            &self.logical_device,
            &self.swap_chain,
            &self.render_pass,
            &self.framebuffers,
            &pipeline,
            &descriptor_pool,
        );
        let sync_objects = SyncObjects::new(&self.logical_device);
        let images_in_flight = vec![vk::Fence::null(); self.swap_chain.images.len()];

        Vulkan {
            _entry: self._entry,
            window: self.window,
            instance: self.instance,
            surface: self.surface,
            physical_device: self.physical_device,
            logical_device: self.logical_device,
            swap_chain: self.swap_chain,
            image_views: self.image_views,
            queues: self.queues,
            render_pass: self.render_pass,
            descriptor_set_layout,
            pipeline,
            framebuffers: self.framebuffers,
            command_pool: self.command_pool,
            descriptor_pool,
            sync_objects,
            images_in_flight,
            framebuffer_resized: false,
            memory_properties: self.memory_properties,
            uniforms: self.uniforms,
            textures: self.textures,
            dynamic_textures: self.dynamic_textures,
            sampler,
            image_index: 0,
        }
    }

    fn check_validation_layers_support(entry: &ash::Entry) -> bool {
        let available_layers = entry
            .enumerate_instance_layer_properties()
            .expect("Failed to enumerate instance layer properties");

        for layer in &constants::VALIDATION_LAYERS {
            let mut found = false;

            for available_layer in &available_layers {
                let layer_name =
                    unsafe { std::ffi::CStr::from_ptr(available_layer.layer_name.as_ptr()) }
                        .to_str()
                        .unwrap();

                if *layer == layer_name {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }
}

pub struct Vulkan {
    _entry: ash::Entry,
    window: Rc<Window>,
    instance: Instance,
    surface: Surface,
    physical_device: PhysicalDevice,
    logical_device: LogicalDevice,
    swap_chain: SwapChain,
    image_views: ImageViews,
    queues: Queues,
    render_pass: RenderPass,
    descriptor_set_layout: DescriptorSetLayout,
    pipeline: Pipeline,
    framebuffers: Framebuffers,
    command_pool: CommandPool,
    descriptor_pool: DescriptorPool,
    sync_objects: SyncObjects,
    images_in_flight: Vec<vk::Fence>,
    framebuffer_resized: bool,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    uniforms: UniformLayouts<UniformBuffers>,
    textures: UniformLayouts<StaticTexture>,
    dynamic_textures: UniformLayouts<DynamicTexture>,
    sampler: Sampler,
    image_index: usize,
}

impl Vulkan {
    pub fn builder(window: Rc<Window>) -> VulkanBuilder {
        VulkanBuilder::new(window)
    }

    pub fn begin_draw(&mut self) {
        unsafe {
            self.logical_device.value.wait_for_fences(
                &[self.sync_objects.in_flight()],
                true,
                u64::MAX,
            )
        }
        .expect("Failed to wait for in flight fence");

        let result = unsafe {
            self.swap_chain.loader.acquire_next_image(
                self.swap_chain.value,
                u64::MAX,
                self.sync_objects.image_available(),
                vk::Fence::null(),
            )
        };

        self.image_index = match result {
            Ok(result) => result.0,
            Err(result) => match result {
                vk::Result::ERROR_OUT_OF_DATE_KHR => {
                    self.recreate_swap_chain();
                    return;
                }
                _ => panic!("Failed to acquire next swap chain image"),
            },
        } as usize;

        if self.images_in_flight[self.image_index] != vk::Fence::null() {
            unsafe {
                self.logical_device.value.wait_for_fences(
                    &[self.images_in_flight[self.image_index]],
                    true,
                    u64::MAX,
                )
            }
            .expect("Failed to wait for in flight fence");
        }

        self.images_in_flight[self.image_index] = self.sync_objects.in_flight();
    }

    pub fn end_draw(&mut self) {
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: &self.sync_objects.image_available(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &self.command_pool.buffers[self.image_index as usize],
            signal_semaphore_count: 1,
            p_signal_semaphores: &self.sync_objects.render_finished(),
        };

        unsafe {
            self.logical_device
                .value
                .reset_fences(&[self.sync_objects.in_flight()])
        }
        .expect("Failed to reset in flight fence");

        unsafe {
            self.logical_device.value.queue_submit(
                self.queues.graphics,
                &[submit_info],
                self.sync_objects.in_flight(),
            )
        }
        .expect("Failed to submit draw command buffer");

        let present_info = vk::PresentInfoKHR {
            s_type: vk::StructureType::PRESENT_INFO_KHR,
            p_next: ptr::null(),
            wait_semaphore_count: 1,
            p_wait_semaphores: &self.sync_objects.render_finished(),
            swapchain_count: 1,
            p_swapchains: &self.swap_chain.value,
            p_image_indices: &(self.image_index as u32),
            p_results: ptr::null_mut(),
        };

        let result = unsafe {
            self.swap_chain
                .loader
                .queue_present(self.queues.present, &present_info)
        };

        let resized = match result {
            Ok(result) => result,
            Err(result)
                if result == vk::Result::ERROR_OUT_OF_DATE_KHR
                    || result == vk::Result::SUBOPTIMAL_KHR =>
            {
                true
            }
            _ => {
                panic!("Failed to present swap chain image");
            }
        };

        if resized || self.framebuffer_resized {
            self.framebuffer_resized = false;
            self.recreate_swap_chain();
        }

        self.sync_objects.increment();
    }

    pub fn update_uniform<T>(&mut self, binding: u32, value: T) {
        let data = unsafe {
            self.logical_device.value.map_memory(
                self.uniforms[&binding].uniforms.memory(self.image_index),
                0,
                self.uniforms[&binding].uniforms.size,
                vk::MemoryMapFlags::empty(),
            )
        }
        .expect("Failed to map memory") as *mut T;

        let buffers = [value];

        unsafe {
            data.copy_from_nonoverlapping(buffers.as_ptr(), buffers.len());
            self.logical_device
                .value
                .unmap_memory(self.uniforms[&binding].uniforms.memory(self.image_index));
        }
    }

    pub fn update_texture<T>(&mut self, binding: u32, data: &Vec<T>) {
        self.dynamic_textures.get_mut(&binding).unwrap().uniforms.update(
            &self.logical_device,
            &self.command_pool,
            &self.queues,
            data,
        );
    }

    pub fn framebuffer_resized(&mut self) {
        self.framebuffer_resized = true;
    }

    fn cleanup_swap_chain(&mut self) {
        self.command_pool.free_buffers(&self.logical_device);
        self.framebuffers.destroy(&self.logical_device);
        self.pipeline.destroy(&self.logical_device);
        self.render_pass.destroy(&self.logical_device);
        self.image_views.destroy(&self.logical_device);
        self.swap_chain.destroy();
        for uniform in self.uniforms.values_mut() {
            uniform.uniforms.destroy(&self.logical_device);
        }
        self.descriptor_pool.destroy(&self.logical_device);
    }

    fn recreate_swap_chain(&mut self) {
        unsafe { self.logical_device.value.device_wait_idle() }
            .expect("Failed to wait for device idle");

        self.cleanup_swap_chain();
        self.swap_chain = SwapChain::new(
            &self.instance,
            &self.surface,
            &self.physical_device,
            &self.logical_device,
            self.window.as_ref(),
        );
        self.image_views = ImageViews::new(&self.logical_device, &self.swap_chain);
        self.queues = Queues::new(&self.logical_device, &self.physical_device.indices);
        self.render_pass = RenderPass::new(&self.logical_device, &self.swap_chain);
        self.pipeline = Pipeline::new(
            &self.logical_device,
            &self.swap_chain,
            &self.render_pass,
            &self.descriptor_set_layout,
        );
        self.framebuffers = Framebuffers::new(
            &self.logical_device,
            &self.swap_chain,
            &self.image_views,
            &self.render_pass,
        );
        for uniform in self.uniforms.values_mut() {
            uniform.uniforms = UniformBuffers::new(
                self.memory_properties,
                &self.logical_device,
                self.swap_chain.images.len(),
                uniform.uniforms.size,
            );
        }
        self.descriptor_pool = DescriptorPool::new(
            &self.logical_device,
            self.swap_chain.images.len(),
            &self.descriptor_set_layout,
            &self.uniforms,
            &self.textures,
            &self.dynamic_textures,
            &self.sampler,
        );
        self.command_pool.create_buffers(
            &self.logical_device,
            &self.swap_chain,
            &self.render_pass,
            &self.framebuffers,
            &self.pipeline,
            &self.descriptor_pool,
        );
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe { self.logical_device.value.device_wait_idle() }
            .expect("Failed to wait for device idle");
        self.cleanup_swap_chain();
        self.sync_objects.destroy(&self.logical_device);
        self.sampler.destroy(&self.logical_device);
        for texture in self.textures.values_mut() {
            texture.uniforms.destroy(&self.logical_device);
        }
        for texture in self.dynamic_textures.values_mut() {
            texture.uniforms.destroy(&self.logical_device);
        }
        self.command_pool.destroy(&self.logical_device);
        self.descriptor_set_layout.destroy(&self.logical_device);
        self.logical_device.destroy();
        self.surface.destroy();
        self.instance.destroy();
    }
}
