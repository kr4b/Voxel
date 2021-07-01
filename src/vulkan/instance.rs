use ash::version::{EntryV1_0, InstanceV1_0};
use ash::vk;

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use super::{constants, platform};

pub struct Instance {
    pub value: ash::Instance,
}

impl Instance {
    pub fn new(entry: &ash::Entry) -> Self {
        let app_name = CString::new(constants::WINDOW_TITLE).unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_version(1, 0, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_version(1, 0, 0),
            api_version: vk::API_VERSION_1_0,
        };

        let extension_names = platform::required_extension_names();
        let validation_layers: Vec<CString> = constants::VALIDATION_LAYERS
            .iter()
            .map(|x| CString::new(*x).unwrap())
            .collect();
        let raw_validation_layers: Vec<*const c_char> =
            validation_layers.iter().map(|x| x.as_ptr()).collect();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: if constants::ENABLE_VALIDATION {
                raw_validation_layers.as_ptr()
            } else {
                ptr::null()
            },
            enabled_layer_count: if constants::ENABLE_VALIDATION {
                raw_validation_layers.len() as u32
            } else {
                0
            },
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let value = unsafe { entry.create_instance(&create_info, None) }
            .expect("Failed to create instance");

        Self { value }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.value.destroy_instance(None);
        }
    }
}
