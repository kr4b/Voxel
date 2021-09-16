use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::util::{create_buffer, find_memory_type};
use super::{CommandPool, ImageViews, LogicalDevice, Queues};

type TextureComponents = (
    vk::Image,
    vk::DeviceMemory,
    vk::ImageView,
    vk::Buffer,
    vk::DeviceMemory,
);

pub struct StaticTexture {
    pub value: vk::Image,
    pub memory: vk::DeviceMemory,
    pub image_view: vk::ImageView,
}

pub struct DynamicTexture {
    pub buffer: vk::Buffer,
    pub buffer_memory: vk::DeviceMemory,
    pub value: vk::Image,
    pub memory: vk::DeviceMemory,
    pub image_view: vk::ImageView,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub dimensions: u32,
}

fn new_1d(
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    logical_device: &LogicalDevice,
    command_pool: &CommandPool,
    queues: &Queues,
    width: u32,
    dimensions: u32,
    format: vk::Format,
) -> TextureComponents {
    new(
        memory_properties,
        logical_device,
        command_pool,
        queues,
        width,
        1,
        1,
        vk::ImageType::TYPE_1D,
        vk::ImageViewType::TYPE_1D,
        dimensions,
        format,
    )
}

fn new_2d(
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    logical_device: &LogicalDevice,
    command_pool: &CommandPool,
    queues: &Queues,
    width: u32,
    height: u32,
    dimensions: u32,
    format: vk::Format,
) -> TextureComponents {
    new(
        memory_properties,
        logical_device,
        command_pool,
        queues,
        width,
        height,
        1,
        vk::ImageType::TYPE_2D,
        vk::ImageViewType::TYPE_2D,
        dimensions,
        format,
    )
}

fn new_3d(
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    logical_device: &LogicalDevice,
    command_pool: &CommandPool,
    queues: &Queues,
    width: u32,
    height: u32,
    depth: u32,
    dimensions: u32,
    format: vk::Format,
) -> TextureComponents {
    new(
        memory_properties,
        logical_device,
        command_pool,
        queues,
        width,
        height,
        depth,
        vk::ImageType::TYPE_3D,
        vk::ImageViewType::TYPE_3D,
        dimensions,
        format,
    )
}

fn new(
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    logical_device: &LogicalDevice,
    command_pool: &CommandPool,
    queues: &Queues,
    width: u32,
    height: u32,
    depth: u32,
    image_type: vk::ImageType,
    view_type: vk::ImageViewType,
    dimensions: u32,
    format: vk::Format,
) -> TextureComponents {
    let size = (width * height * depth * dimensions) as vk::DeviceSize;
    let (buffer, buffer_memory) = create_buffer(
        memory_properties,
        logical_device,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    );

    let create_info = vk::ImageCreateInfo {
        s_type: vk::StructureType::IMAGE_CREATE_INFO,
        p_next: ptr::null(),
        flags: vk::ImageCreateFlags::empty(),
        image_type,
        format,
        extent: vk::Extent3D {
            width,
            height,
            depth,
        },
        mip_levels: 1,
        array_layers: 1,
        samples: vk::SampleCountFlags::TYPE_1,
        tiling: vk::ImageTiling::OPTIMAL,
        usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: ptr::null(),
        initial_layout: vk::ImageLayout::UNDEFINED,
    };

    let value = unsafe { logical_device.value.create_image(&create_info, None) }
        .expect("Failed to create image");

    let memory_requirements = unsafe { logical_device.value.get_image_memory_requirements(value) };
    let allocate_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next: ptr::null(),
        allocation_size: memory_requirements.size,
        memory_type_index: find_memory_type(
            memory_properties,
            memory_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ),
    };

    let memory = unsafe { logical_device.value.allocate_memory(&allocate_info, None) }
        .expect("Failed to allocate image memory");
    unsafe { logical_device.value.bind_image_memory(value, memory, 0) }
        .expect("Failed to bind image memory");

    let image_view = ImageViews::create_image_view(logical_device, value, format, view_type);

    (value, memory, image_view, buffer, buffer_memory)
}

fn copy_buffer_to_image(
    logical_device: &LogicalDevice,
    command_buffer: vk::CommandBuffer,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32,
    depth: u32,
) {
    let region = vk::BufferImageCopy {
        buffer_offset: 0,
        buffer_row_length: 0,
        buffer_image_height: 0,
        image_subresource: vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        },
        image_offset: Default::default(),
        image_extent: vk::Extent3D {
            width,
            height,
            depth,
        },
    };

    unsafe {
        logical_device.value.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[region],
        );
    }
}

fn transition_image_layout(
    logical_device: &LogicalDevice,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
) {
    let (src_stages, dst_stages, src_access_mask, dst_access_mask) = match (old_layout, new_layout)
    {
        (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::empty(),
            vk::AccessFlags::TRANSFER_WRITE,
        ),
        (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::AccessFlags::TRANSFER_WRITE,
            vk::AccessFlags::SHADER_READ,
        ),
        (vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (
            vk::PipelineStageFlags::FRAGMENT_SHADER,
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::SHADER_READ,
            vk::AccessFlags::TRANSFER_WRITE,
        ),
        _ => panic!("Unsupported image layout transition"),
    };

    let barrier = vk::ImageMemoryBarrier {
        s_type: vk::StructureType::IMAGE_MEMORY_BARRIER,
        p_next: ptr::null(),
        src_access_mask,
        dst_access_mask,
        old_layout,
        new_layout,
        src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        image,
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
    };

    unsafe {
        logical_device.value.cmd_pipeline_barrier(
            command_buffer,
            src_stages,
            dst_stages,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[barrier],
        );
    }
}

fn load_data<T>(
    logical_device: &LogicalDevice,
    buffer_memory: vk::DeviceMemory,
    width: u32,
    height: u32,
    depth: u32,
    dimensions: u32,
    data: &Vec<T>,
) {
    let size = (width * height * depth * dimensions) as vk::DeviceSize;
    let raw_data = unsafe {
        logical_device
            .value
            .map_memory(buffer_memory, 0, size, vk::MemoryMapFlags::empty())
    }
    .expect("Failed to map memory") as *mut T;

    unsafe {
        raw_data.copy_from_nonoverlapping(data.as_ptr(), data.len());
        logical_device.value.unmap_memory(buffer_memory);
    }
}

impl StaticTexture {
    pub fn new_3d<T>(
        memory_properties: vk::PhysicalDeviceMemoryProperties,
        logical_device: &LogicalDevice,
        command_pool: &CommandPool,
        queues: &Queues,
        width: u32,
        height: u32,
        depth: u32,
        dimensions: u32,
        format: vk::Format,
        data: &Vec<T>,
    ) -> Self {
        let (value, memory, image_view, buffer, buffer_memory) = new_3d(
            memory_properties,
            logical_device,
            command_pool,
            queues,
            width,
            height,
            depth,
            dimensions,
            format,
        );

        load_data(logical_device, buffer_memory, width, height, depth, dimensions, data);

        let command_buffer = command_pool.begin_single_time_commands(logical_device);
        transition_image_layout(
            logical_device,
            command_buffer,
            value,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        copy_buffer_to_image(
            logical_device,
            command_buffer,
            buffer,
            value,
            width,
            height,
            depth,
        );
        transition_image_layout(
            logical_device,
            command_buffer,
            value,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
        command_pool.end_single_time_commands(logical_device, command_buffer, queues.graphics);

        unsafe {
            logical_device.value.destroy_buffer(buffer, None);
            logical_device.value.free_memory(buffer_memory, None);
        }

        Self {
            value,
            memory,
            image_view,
        }
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device.value.destroy_image_view(self.image_view, None);
            logical_device.value.destroy_image(self.value, None);
            logical_device.value.free_memory(self.memory, None);
        }
    }
}

impl DynamicTexture {
    pub fn new_3d(
        memory_properties: vk::PhysicalDeviceMemoryProperties,
        logical_device: &LogicalDevice,
        command_pool: &CommandPool,
        queues: &Queues,
        width: u32,
        height: u32,
        depth: u32,
        dimensions: u32,
        format: vk::Format,
) -> Self {
        let (value, memory, image_view, buffer, buffer_memory) = new_3d(
            memory_properties,
            logical_device,
            command_pool,
            queues,
            width,
            height,
            depth,
            dimensions,
            format,
        );
        let command_buffer = command_pool.begin_single_time_commands(logical_device);
        transition_image_layout(
            logical_device,
            command_buffer,
            value,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        transition_image_layout(
            logical_device,
            command_buffer,
            value,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
        command_pool.end_single_time_commands(logical_device, command_buffer, queues.graphics);

        Self {
            value,
            memory,
            image_view,
            buffer,
            buffer_memory,
            width,
            height,
            depth,
            dimensions,
        }
    }

    pub fn update<T>(&self, logical_device: &LogicalDevice, command_pool: &CommandPool, queues: &Queues, data: &Vec<T>) {
        load_data(logical_device, self.buffer_memory, self.width, self.height, self.depth, self.dimensions, data);

        let command_buffer = command_pool.begin_single_time_commands(logical_device);
        transition_image_layout(
            logical_device,
            command_buffer,
            self.value,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        );
        copy_buffer_to_image(
            logical_device,
            command_buffer,
            self.buffer,
            self.value,
            self.width,
            self.height,
            self.depth,
        );
        transition_image_layout(
            logical_device,
            command_buffer,
            self.value,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
        command_pool.end_single_time_commands(logical_device, command_buffer, queues.graphics);
    }

    pub fn destroy(&self, logical_device: &LogicalDevice) {
        unsafe {
            logical_device.value.destroy_buffer(self.buffer, None);
            logical_device.value.free_memory(self.buffer_memory, None);
            logical_device.value.destroy_image_view(self.image_view, None);
            logical_device.value.destroy_image(self.value, None);
            logical_device.value.free_memory(self.memory, None);
        }
    }
}
