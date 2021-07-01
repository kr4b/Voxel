use std::ptr;

use ash::vk;
use winit::dpi::LogicalSize;

use super::instance::Instance;
use super::logical_device::LogicalDevice;
use super::physical_device::PhysicalDevice;
use super::surface::Surface;
use super::swap_chain_support::SwapChainSupport;

pub struct SwapChain {
    pub loader: ash::extensions::khr::Swapchain,
    pub value: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_format: vk::Format,
    pub extent: vk::Extent2D,
}

impl SwapChain {
    pub fn new(
        instance: &Instance,
        surface: &Surface,
        physical_device: &PhysicalDevice,
        logical_device: &LogicalDevice,
        window: &winit::window::Window,
    ) -> Self {
        let window_size = window.inner_size().to_logical(window.scale_factor());
        let swap_chain_support = Self::query_swap_chain_support(surface, physical_device);
        let format = Self::select_swap_surface_format(&swap_chain_support.formats);
        let present_mode = Self::select_swap_present_mode(&swap_chain_support.present_modes);
        let extent = Self::select_swap_extent(&swap_chain_support.capabilities, window_size);

        let mut image_count = swap_chain_support.capabilities.min_image_count + 1;
        let max_image_count = swap_chain_support.capabilities.max_image_count;
        if max_image_count > 0 && image_count > max_image_count {
            image_count = max_image_count;
        }

        let same_indices =
            physical_device.indices.graphics_family == physical_device.indices.present_family;
        let queue_family_indices = physical_device.indices.queue_families();

        let swap_chain_create_info = vk::SwapchainCreateInfoKHR {
            s_type: vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ptr::null(),
            flags: vk::SwapchainCreateFlagsKHR::empty(),
            surface: surface.value,
            min_image_count: image_count,
            image_format: format.format,
            image_color_space: format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: if !same_indices {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            },
            queue_family_index_count: if !same_indices { 2 } else { 0 },
            p_queue_family_indices: if !same_indices {
                queue_family_indices.as_ptr()
            } else {
                ptr::null()
            },
            pre_transform: swap_chain_support.capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode: present_mode,
            clipped: vk::TRUE,
            old_swapchain: vk::SwapchainKHR::null(),
        };

        let loader = ash::extensions::khr::Swapchain::new(&instance.value, &logical_device.value);
        let value = unsafe { loader.create_swapchain(&swap_chain_create_info, None) }
            .expect("Failed to create swapchain");

        let images =
            unsafe { loader.get_swapchain_images(value) }.expect("Failed to get swapchain images");

        Self {
            value,
            loader,
            images,
            image_format: format.format,
            extent,
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.loader.destroy_swapchain(self.value, None);
        }
    }

    fn select_swap_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        size: LogicalSize<u32>,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            vk::Extent2D {
                width: size
                    .width
                    .min(capabilities.max_image_extent.width)
                    .max(capabilities.min_image_extent.width),
                height: size
                    .height
                    .min(capabilities.max_image_extent.height)
                    .max(capabilities.min_image_extent.height),
            }
        }
    }

    fn select_swap_present_mode(present_modes: &Vec<vk::PresentModeKHR>) -> vk::PresentModeKHR {
        for &present_mode in present_modes {
            if present_mode == vk::PresentModeKHR::MAILBOX {
                return present_mode;
            }
        }

        vk::PresentModeKHR::FIFO
    }

    fn select_swap_surface_format(formats: &Vec<vk::SurfaceFormatKHR>) -> vk::SurfaceFormatKHR {
        for format in formats {
            if format.format == vk::Format::R8G8B8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *format;
            }
        }

        formats[0]
    }

    fn query_swap_chain_support(
        surface: &Surface,
        physical_device: &PhysicalDevice,
    ) -> SwapChainSupport {
        let capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(physical_device.value, surface.value)
        }
        .expect("Failed to get physical device surface capabilities");

        let formats = unsafe {
            surface
                .loader
                .get_physical_device_surface_formats(physical_device.value, surface.value)
        }
        .expect("Failed to get physical device surface formats");

        let present_modes = unsafe {
            surface
                .loader
                .get_physical_device_surface_present_modes(physical_device.value, surface.value)
        }
        .expect("Failed to get physical device surface present modes");

        SwapChainSupport {
            capabilities,
            formats,
            present_modes,
        }
    }
}
