use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::LogicalDevice;

pub fn create_buffer(
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    logical_device: &LogicalDevice,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    properties: vk::MemoryPropertyFlags,
) -> (vk::Buffer, vk::DeviceMemory) {
    let create_info = vk::BufferCreateInfo {
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size,
        usage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
    };

    let buffer = unsafe { logical_device.value.create_buffer(&create_info, None) }
        .expect("Failed to create buffer");

    let memory_requirements =
        unsafe { logical_device.value.get_buffer_memory_requirements(buffer) };

    let allocate_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: memory_requirements.size,
        memory_type_index: find_memory_type(
            memory_properties,
            memory_requirements.memory_type_bits,
            properties,
        ),
    };

    let buffer_memory = unsafe { logical_device.value.allocate_memory(&allocate_info, None) }
        .expect("Failed to allocate buffer memory");

    unsafe {
        logical_device
            .value
            .bind_buffer_memory(buffer, buffer_memory, 0)
    }.expect("Failed to bind buffer memory");

    (buffer, buffer_memory)
}

pub fn find_memory_type(
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    for i in 0..memory_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && (memory_properties.memory_types[i as usize].property_flags & properties)
                == properties
        {
            return i;
        }
    }

    panic!("Failed to find suitable memory type");
}
