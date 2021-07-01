use ash::version::InstanceV1_0;
use ash::vk;

use super::instance::Instance;
use super::queue_indices::QueueIndices;
use super::surface::Surface;
use super::swap_chain_support::SwapChainSupport;

pub struct PhysicalDevice {
    pub value: vk::PhysicalDevice,
    pub indices: QueueIndices,
}

impl PhysicalDevice {
    pub fn new(instance: &Instance, surface: &Surface) -> Self {
        let available_devices = unsafe { instance.value.enumerate_physical_devices() }
            .expect("Failed to enumerate physical devices");

        debug_assert!(
            !available_devices.is_empty(),
            "No physical devices available"
        );

        let mut best_device = (0, None, None);
        for device in available_devices {
            let indices = QueueIndices::new(&instance, surface, device);

            if Self::is_device_suitable(instance, &indices, surface, device) {
                let score = Self::get_device_score(instance, &indices, device);
                if best_device.1.is_none() || score > best_device.0 {
                    best_device = (score, Some(device), Some(indices));
                }
            }
        }

        Self {
            value: best_device.1.expect("No suitable physical device found"),
            indices: best_device.2.expect("No suitable physical device indices found"),
        }

    }

    fn is_device_suitable(instance: &Instance,
        indices: &QueueIndices,
        surface: &Surface,
        device: vk::PhysicalDevice,
    ) -> bool {
        if !Self::check_device_extensions_support(instance, device) {
            return false;
        }

        let swap_chain_support = SwapChainSupport::new(surface, device);
        let valid_swap_chain =
            !swap_chain_support.formats.is_empty() && !swap_chain_support.present_modes.is_empty();

        indices.is_correct() && valid_swap_chain
    }

    fn get_device_score(
        instance: &Instance,
        indices: &QueueIndices,
        device: vk::PhysicalDevice,
    ) -> usize {
        let mut device_score = 0;
        let properties = unsafe { instance.value.get_physical_device_properties(device) };
        // let features = unsafe { instance.get_physical_device_features(device) };

        if indices.graphics_family == indices.present_family {
            device_score += 1;
        }

        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            device_score += 100;
        }

        device_score
    }

    fn check_device_extensions_support(
        instance: &Instance,
        device: vk::PhysicalDevice,
    ) -> bool {
        let available_extensions =
            unsafe { instance.value.enumerate_device_extension_properties(device) }
                .expect("Failed to enumerate device extension properties");

        let required_extensions = super::constants::get_device_extensions();
        for extension in &required_extensions {
            let mut found = false;

            for available_extension in &available_extensions {
                let extension_name = unsafe {
                    std::ffi::CStr::from_ptr(available_extension.extension_name.as_ptr())
                };

                if *extension == extension_name {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }
}