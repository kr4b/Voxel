use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{
    Buffer, DescriptorSetLayout, DynamicTexture, LogicalDevice, Sampler, StaticTexture,
    BufferLayout, BufferLayouts,
};

pub struct DescriptorPool {
    pub value: vk::DescriptorPool,
    pub sets: Vec<vk::DescriptorSet>,
}

impl DescriptorPool {
    pub fn new(
        logical_device: &LogicalDevice,
        swap_chain_images_len: usize,
        descriptor_set_layout: &DescriptorSetLayout,
        buffers: &BufferLayouts<Buffer>,
        textures: &BufferLayouts<StaticTexture>,
        dynamic_textures: &BufferLayouts<DynamicTexture>,
        sampler: &Sampler,
    ) -> Self {
        let mut sizes = Vec::new();
        for (_, BufferLayout { buffer, .. }) in buffers {
            sizes.push(vk::DescriptorPoolSize {
                ty: buffer.descriptor_type,
                descriptor_count: swap_chain_images_len as u32,
            });
        }

        for _ in 0..textures.len() {
            sizes.push(vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: swap_chain_images_len as u32,
            });
        }

        for _ in 0..dynamic_textures.len() {
            sizes.push(vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: swap_chain_images_len as u32,
            });
        }

        let create_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DescriptorPoolCreateFlags::empty(),
            max_sets: swap_chain_images_len as u32,
            pool_size_count: sizes.len() as u32,
            p_pool_sizes: sizes.as_ptr(),
        };

        let value = unsafe {
            logical_device
                .value
                .create_descriptor_pool(&create_info, None)
        }
        .expect("Failed to create descriptor pool");

        let layouts = vec![descriptor_set_layout.value; swap_chain_images_len];

        let allocate_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ptr::null(),
            descriptor_pool: value,
            descriptor_set_count: swap_chain_images_len as u32,
            p_set_layouts: layouts.as_ptr(),
        };

        let sets = unsafe {
            logical_device
                .value
                .allocate_descriptor_sets(&allocate_info)
        }
        .expect("Failed to allocate descriptor sets");

        for i in 0..swap_chain_images_len {
            let mut descriptor_sets = Vec::new();
            let mut buffer_infos = Vec::new();
            for (binding, BufferLayout { buffer, .. }) in buffers {
                buffer_infos.push(vk::DescriptorBufferInfo {
                    buffer: buffer.buffer(i),
                    offset: 0,
                    range: buffer.size,
                });

                let buffer_set_write = vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: sets[i],
                    dst_binding: *binding,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: buffer.descriptor_type,
                    p_image_info: ptr::null(),
                    p_buffer_info: buffer_infos.last().unwrap(),
                    p_texel_buffer_view: ptr::null(),
                };
                descriptor_sets.push(buffer_set_write);
            }

            let mut texture_infos = Vec::new();
            for (binding, BufferLayout { buffer, .. }) in textures {
                texture_infos.push(vk::DescriptorImageInfo {
                    image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    image_view: buffer.image_view,
                    sampler: sampler.value,
                });

                let sampler_set_write = vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: sets[i],
                    dst_binding: *binding,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    p_image_info: texture_infos.last().unwrap(),
                    p_buffer_info: ptr::null(),
                    p_texel_buffer_view: ptr::null(),
                };
                descriptor_sets.push(sampler_set_write);
            }

            let mut dynamic_texture_infos = Vec::new();
            for (binding, BufferLayout { buffer, .. }) in dynamic_textures {
                dynamic_texture_infos.push(vk::DescriptorImageInfo {
                    image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    image_view: buffer.image_view,
                    sampler: sampler.value,
                });

                let sampler_set_write = vk::WriteDescriptorSet {
                    s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
                    p_next: ptr::null(),
                    dst_set: sets[i],
                    dst_binding: *binding,
                    dst_array_element: 0,
                    descriptor_count: 1,
                    descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    p_image_info: dynamic_texture_infos.last().unwrap(),
                    p_buffer_info: ptr::null(),
                    p_texel_buffer_view: ptr::null(),
                };
                descriptor_sets.push(sampler_set_write);
            }

            unsafe {
                logical_device
                    .value
                    .update_descriptor_sets(&descriptor_sets, &[]);
            }
        }

        Self { value, sets }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device
                .value
                .destroy_descriptor_pool(self.value, None);
        }
    }
}
