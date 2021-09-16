use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{
    Buffer, DynamicTexture, LogicalDevice, StaticTexture, BufferLayout, BufferLayouts,
};

pub struct DescriptorSetLayout {
    pub value: vk::DescriptorSetLayout,
}

impl DescriptorSetLayout {
    pub fn new(
        logical_device: &LogicalDevice,
        buffers: &BufferLayouts<Buffer>,
        textures: &BufferLayouts<StaticTexture>,
        dynamic_textures: &BufferLayouts<DynamicTexture>,
    ) -> Self {
        let mut bindings = Vec::new();
        for (binding, BufferLayout { stage_flags, buffer }) in buffers {
            bindings.push(vk::DescriptorSetLayoutBinding {
                binding: *binding,
                descriptor_type: buffer.descriptor_type,
                descriptor_count: 1,
                stage_flags: *stage_flags,
                p_immutable_samplers: ptr::null(),
            });
        }

        for (binding, BufferLayout { stage_flags, .. }) in textures {
            bindings.push(vk::DescriptorSetLayoutBinding {
                binding: *binding,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: *stage_flags,
                p_immutable_samplers: ptr::null(),
            });
        }

        for (binding, BufferLayout { stage_flags, .. }) in dynamic_textures {
            bindings.push(vk::DescriptorSetLayoutBinding {
                binding: *binding,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: *stage_flags,
                p_immutable_samplers: ptr::null(),
            });
        }

        let layout_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
        };

        let value = unsafe {
            logical_device
                .value
                .create_descriptor_set_layout(&layout_info, None)
        }
        .expect("Failed to create descriptor set layout");

        Self { value }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device
                .value
                .destroy_descriptor_set_layout(self.value, None);
        }
    }
}
