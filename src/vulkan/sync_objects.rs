use std::ptr;

use ash::version::DeviceV1_0;
use ash::vk;

use super::{constants, LogicalDevice};

struct SyncObject {
    image_available: vk::Semaphore,
    render_finished: vk::Semaphore,
    in_flight: vk::Fence,
}

pub struct SyncObjects {
    values: Vec<SyncObject>,
    pub current_frame: usize,
}

impl SyncObjects {
    pub fn new(logical_device: &LogicalDevice) -> Self {
        let semaphore_create_info = vk::SemaphoreCreateInfo {
            s_type: vk::StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::SemaphoreCreateFlags::empty(),
        };

        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        let mut values = Vec::with_capacity(constants::MAX_FRAMES_IN_FLIGHT);

        for _ in 0..constants::MAX_FRAMES_IN_FLIGHT {
            let image_available = unsafe {
                logical_device
                    .value
                    .create_semaphore(&semaphore_create_info, None)
            }
            .expect("Failed to create semaphore");
            let render_finished = unsafe {
                logical_device
                    .value
                    .create_semaphore(&semaphore_create_info, None)
            }
            .expect("Failed to create semaphore");
            let in_flight = unsafe { logical_device.value.create_fence(&fence_create_info, None) }
                .expect("Failed to create fence");

            values.push(SyncObject {
                image_available,
                render_finished,
                in_flight,
            });
        }

        Self {
            values,
            current_frame: 0,
        }
    }

    pub fn image_available(&self) -> vk::Semaphore {
        self.values[self.current_frame].image_available
    }

    pub fn render_finished(&self) -> vk::Semaphore {
        self.values[self.current_frame].render_finished
    }

    pub fn in_flight(&self) -> vk::Fence {
        self.values[self.current_frame].in_flight
    }

    pub fn increment(&mut self) {
        self.current_frame = (self.current_frame + 1) % constants::MAX_FRAMES_IN_FLIGHT;
    }

    pub fn destroy(&mut self, logical_device: &LogicalDevice) {
        for i in 0..constants::MAX_FRAMES_IN_FLIGHT {
            unsafe {
                logical_device
                    .value
                    .destroy_semaphore(self.values[i].image_available, None);
                logical_device
                    .value
                    .destroy_semaphore(self.values[i].render_finished, None);
                logical_device
                    .value
                    .destroy_fence(self.values[i].in_flight, None);
            }
        }
    }
}
