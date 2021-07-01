use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{LogicalDevice, SwapChain};

pub struct ImageViews {
    pub values: Vec<vk::ImageView>,
}

impl ImageViews {
    pub fn new(logical_device: &LogicalDevice, swap_chain: &SwapChain) -> Self {
        let mut values = Vec::with_capacity(swap_chain.images.len());

        for &image in &swap_chain.images {
            values.push(Self::create_image_view(
                logical_device,
                image,
                swap_chain.image_format,
                vk::ImageViewType::TYPE_2D,
            ));
        }

        Self { values }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        for image_view in self.values.drain(..) {
            unsafe {
                logical_device.value.destroy_image_view(image_view, None);
            }
        }
    }

    pub fn create_image_view(
        logical_device: &LogicalDevice,
        image: vk::Image,
        format: vk::Format,
        view_type: vk::ImageViewType,
    ) -> vk::ImageView {
        let image_view_create_info = vk::ImageViewCreateInfo {
            s_type: vk::StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::ImageViewCreateFlags::empty(),
            image,
            view_type,
            format,
            components: Default::default(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
        };

        unsafe {
            logical_device
                .value
                .create_image_view(&image_view_create_info, None)
        }
        .expect("Failed to create image view")
    }
}
