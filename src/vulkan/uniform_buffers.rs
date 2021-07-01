use ash::version::DeviceV1_0;
use ash::vk;

use super::util::create_buffer;
use super::LogicalDevice;

pub struct UniformBuffers {
    values: Vec<vk::Buffer>,
    memory: Vec<vk::DeviceMemory>,
    pub size: vk::DeviceSize,
}

impl UniformBuffers {
    pub fn new(
        memory_properties: vk::PhysicalDeviceMemoryProperties,
        logical_device: &LogicalDevice,
        swap_chain_images_len: usize,
        size: vk::DeviceSize,
    ) -> UniformBuffers {
        let mut values = Vec::with_capacity(swap_chain_images_len);
        let mut memory = Vec::with_capacity(swap_chain_images_len);

        for _ in 0..swap_chain_images_len {
            let (buffer, buffer_memory) = create_buffer(
                memory_properties,
                logical_device,
                size,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );
            values.push(buffer);
            memory.push(buffer_memory);
        }

        UniformBuffers {
            values,
            memory,
            size,
        }
    }

    pub fn buffer(&self, current_image: usize) -> vk::Buffer {
        self.values[current_image]
    }

    pub fn memory(&self, current_image: usize) -> vk::DeviceMemory {
        self.memory[current_image]
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        for buffer in self.values.drain(..) {
            unsafe {
                logical_device.value.destroy_buffer(buffer, None);
            }
        }
        for memory in self.memory.drain(..) {
            unsafe {
                logical_device.value.free_memory(memory, None);
            }
        }
    }
}
