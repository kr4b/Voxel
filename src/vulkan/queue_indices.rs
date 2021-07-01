use ash::version::InstanceV1_0;
use ash::vk;

use super::instance::Instance;
use super::surface::Surface;

pub struct QueueIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueIndices {
    pub fn new(
        instance: &Instance,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> Self {
        let queue_families =
            unsafe { instance.value.get_physical_device_queue_family_properties(device) };

        let mut indices = Self {
            graphics_family: None,
            present_family: None,
        };

        for (i, queue_family) in queue_families.into_iter().enumerate() {
            let i = i as u32;

            let valid_graphics = queue_family
                .queue_flags
                .contains(ash::vk::QueueFlags::GRAPHICS);
            let valid_present = unsafe {
                surface
                    .loader
                    .get_physical_device_surface_support(device, i, surface.value)
            }
            .expect("Failed to get physical device surface support");

            if valid_graphics && (valid_present || indices.graphics_family.is_none()){
                indices.graphics_family = Some(i);
            }

            if valid_present && (valid_graphics || indices.graphics_family.is_none()) {
                indices.present_family = Some(i);
            }
        }

        indices
    }

    pub fn is_correct(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    pub fn queue_families(&self) -> Vec<u32> {
        let mut unique_families = vec![self.graphics_family.unwrap()];
        if !unique_families.contains(&self.present_family.unwrap()) {
            unique_families.push(self.present_family.unwrap());
        }
        unique_families
    }
}
