use ash::version::DeviceV1_0;
use ash::vk;

use super::LogicalDevice;
use super::queue_indices::QueueIndices;

pub struct Queues {
    pub graphics: vk::Queue,
    pub present: vk::Queue,
}

impl Queues {
    pub fn new(logical_device: &LogicalDevice, indices: &QueueIndices) -> Self {
        let graphics =
            unsafe { logical_device.value.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present = unsafe { logical_device.value.get_device_queue(indices.present_family.unwrap(), 0) };

        Self {
            graphics,
            present
        }
    }
}