use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::vk;

use super::constants;
use super::instance::Instance;

pub struct LogicalDevice {
    pub value: ash::Device,
}

impl LogicalDevice {
    pub fn new(
        instance: &Instance,
        physical_device: &super::physical_device::PhysicalDevice,
    ) -> Self {
        let queue_priorties = [1.0];

        let mut queue_create_info = Vec::new();
        for queue in physical_device.indices.queue_families().into_iter() {
            queue_create_info.push(vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
                p_next: ptr::null(),
                flags: vk::DeviceQueueCreateFlags::empty(),
                queue_family_index: queue,
                queue_count: 1,
                p_queue_priorities: queue_priorties.as_ptr(),
            });
        }

        let device_features = vk::PhysicalDeviceFeatures {
            ..Default::default()
        };

        let validation_layers: Vec<CString> = constants::VALIDATION_LAYERS
            .iter()
            .map(|x| CString::new(*x).unwrap())
            .collect();
        let raw_validation_layers: Vec<*const c_char> =
            validation_layers.iter().map(|x| x.as_ptr()).collect();

        let device_extensions: Vec<*const c_char> = constants::get_device_extensions()
            .iter()
            .map(|x| x.as_ptr())
            .collect();

        let device_create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: queue_create_info.len() as u32,
            p_queue_create_infos: queue_create_info.as_ptr(),
            enabled_layer_count: if constants::ENABLE_VALIDATION {
                constants::VALIDATION_LAYERS.len() as u32
            } else {
                0
            },
            pp_enabled_layer_names: if constants::ENABLE_VALIDATION {
                raw_validation_layers.as_ptr()
            } else {
                ptr::null()
            },
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            p_enabled_features: &device_features,
        };

        let value =
            unsafe { instance.value.create_device(physical_device.value, &device_create_info, None) }
                .expect("Failed to create physical device");

        Self { value }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.value.destroy_device(None);
        }
    }
}
