use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{
    queue_indices::QueueIndices, DescriptorPool, Framebuffers, LogicalDevice, Pipeline, RenderPass,
    SwapChain,
};

pub struct CommandPool {
    pub value: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>,
}

impl CommandPool {
    pub fn new(logical_device: &LogicalDevice, indices: &QueueIndices) -> Self {
        let create_info = vk::CommandPoolCreateInfo {
            s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::CommandPoolCreateFlags::empty(),
            queue_family_index: indices.graphics_family.unwrap(),
        };

        let value = unsafe { logical_device.value.create_command_pool(&create_info, None) }
            .expect("Failed to create command pool");

        Self {
            value,
            buffers: Vec::with_capacity(0),
        }
    }

    pub fn create_buffers(
        &mut self,
        logical_device: &LogicalDevice,
        swap_chain: &SwapChain,
        render_pass: &RenderPass,
        framebuffers: &Framebuffers,
        pipeline: &Pipeline,
        descriptor_pool: &DescriptorPool,
    ) {
        let buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.value,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: framebuffers.values.len() as u32,
        };

        self.buffers = unsafe {
            logical_device
                .value
                .allocate_command_buffers(&buffer_allocate_info)
        }
        .expect("Failed to allocate command buffers");

        for ((buffer, framebuffer), descriptor_set) in self
            .buffers
            .iter()
            .zip(framebuffers.values.iter())
            .zip(descriptor_pool.sets.iter())
        {
            let buffer_begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
                p_next: ptr::null(),
                flags: vk::CommandBufferUsageFlags::empty(),
                p_inheritance_info: ptr::null(),
            };

            unsafe {
                logical_device
                    .value
                    .begin_command_buffer(*buffer, &buffer_begin_info)
            }
            .expect("Failed to begin command buffer");

            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            };
            let render_area = vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: swap_chain.extent,
            };

            let render_pass_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
                p_next: ptr::null(),
                render_pass: render_pass.value,
                framebuffer: *framebuffer,
                render_area,
                clear_value_count: 1,
                p_clear_values: &clear_value,
            };

            let viewport = vk::Viewport {
                x: 0.0,
                y: render_area.extent.height as f32,
                width: render_area.extent.width as f32,
                height: render_area.extent.height as f32 * -1.0,
                min_depth: 0.0,
                max_depth: 1.0,
            };

            unsafe {
                logical_device.value.cmd_begin_render_pass(
                    *buffer,
                    &render_pass_info,
                    vk::SubpassContents::INLINE,
                );
                logical_device.value.cmd_bind_pipeline(
                    *buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.value,
                );
                logical_device.value.cmd_set_viewport(*buffer, 0, &[viewport]);
                logical_device.value.cmd_bind_descriptor_sets(
                    *buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.layout,
                    0,
                    &[*descriptor_set],
                    &[],
                );
                logical_device.value.cmd_draw(*buffer, 3, 1, 0, 0);
                logical_device.value.cmd_end_render_pass(*buffer);
                logical_device
                    .value
                    .end_command_buffer(*buffer)
                    .expect("Failed to end command buffer");
            }
        }
    }

    pub fn begin_single_time_commands(&self, logical_device: &LogicalDevice) -> vk::CommandBuffer {
        let buffer_allocate_info = vk::CommandBufferAllocateInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ptr::null(),
            command_pool: self.value,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: 1,
        };

        let buffer = unsafe {
            logical_device
                .value
                .allocate_command_buffers(&buffer_allocate_info)
        }
        .expect("Failed to allocate command buffers")[0];

        let buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            p_inheritance_info: ptr::null(),
        };

        unsafe {
            logical_device
                .value
                .begin_command_buffer(buffer, &buffer_begin_info)
        }
        .expect("Failed to begin command buffer");

        buffer
    }

    pub fn end_single_time_commands(
        &self,
        logical_device: &LogicalDevice,
        buffer: vk::CommandBuffer,
        graphics_queue: vk::Queue,
    ) {
        unsafe { logical_device.value.end_command_buffer(buffer) }
            .expect("Failed to end command buffer");

        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SUBMIT_INFO,
            command_buffer_count: 1,
            p_command_buffers: &buffer,
            ..Default::default()
        };

        unsafe {
            logical_device
                .value
                .queue_submit(graphics_queue, &[submit_info], vk::Fence::null())
        }
        .expect("Failed to submit draw command buffer");

        unsafe { logical_device.value.queue_wait_idle(graphics_queue) }
            .expect("Failed to wait for graphics queue idle");

        unsafe {
            logical_device
                .value
                .free_command_buffers(self.value, &[buffer]);
        }
    }

    pub fn free_buffers(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device
                .value
                .free_command_buffers(self.value, &self.buffers);
        }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device.value.destroy_command_pool(self.value, None);
        }
    }
}
