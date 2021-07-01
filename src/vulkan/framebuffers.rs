use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{ImageViews, LogicalDevice, RenderPass, SwapChain};

pub struct Framebuffers {
    pub values: Vec<vk::Framebuffer>,
}

impl Framebuffers {
    pub fn new(
        logical_device: &LogicalDevice,
        swap_chain: &SwapChain,
        image_views: &ImageViews,
        render_pass: &RenderPass,
    ) -> Self {
        let values = image_views
            .values
            .iter()
            .map(|image| {
                let framebuffer_create_info = vk::FramebufferCreateInfo {
                    s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
                    p_next: ptr::null(),
                    flags: vk::FramebufferCreateFlags::empty(),
                    render_pass: render_pass.value,
                    attachment_count: 1,
                    p_attachments: image,
                    width: swap_chain.extent.width,
                    height: swap_chain.extent.height,
                    layers: 1,
                };

                unsafe {
                    logical_device
                        .value
                        .create_framebuffer(&framebuffer_create_info, None)
                }
                .expect("Failed to create framebuffer")
            })
            .collect();

        Self { values }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        for framebuffer in self.values.drain(..) {
            unsafe {
                logical_device.value.destroy_framebuffer(framebuffer, None);
            }
        }
    }
}
