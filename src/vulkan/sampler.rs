use std::ptr;

use ash::version::{DeviceV1_0};
use ash::vk;

use super::LogicalDevice;

pub struct Sampler {
    pub value: vk::Sampler,
}

impl Sampler {
    pub fn new(logical_device: &LogicalDevice) -> Sampler {
        let create_info = vk::SamplerCreateInfo {
            s_type: vk::StructureType::SAMPLER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SamplerCreateFlags::empty(),
            mag_filter: vk::Filter::NEAREST,
            min_filter: vk::Filter::NEAREST,
            mipmap_mode: vk::SamplerMipmapMode::NEAREST,
            address_mode_u: vk::SamplerAddressMode::CLAMP_TO_BORDER,
            address_mode_v: vk::SamplerAddressMode::CLAMP_TO_BORDER,
            address_mode_w: vk::SamplerAddressMode::CLAMP_TO_BORDER,
            mip_lod_bias: 0.0,
            anisotropy_enable: vk::FALSE,
            max_anisotropy: 0.0,
            compare_enable: vk::FALSE,
            compare_op: vk::CompareOp::ALWAYS,
            min_lod: 0.0,
            max_lod: 0.0,
            border_color: vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates: vk::FALSE,
        };

        let value = unsafe {
            logical_device.value.create_sampler(&create_info, None)
        }.expect("Failed to create texture sampler");

        Sampler {
            value
        }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device.value.destroy_sampler(self.value, None);
        }
    }
}