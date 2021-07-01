pub const WINDOW_TITLE: &'static str = "Vulkan Test";
pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;
pub const MAX_FRAMES_IN_FLIGHT: usize = 2;

pub const VALIDATION_LAYERS: [&str; 1] = ["VK_LAYER_KHRONOS_validation"];

pub const ENABLE_VALIDATION: bool = cfg!(debug_assertions);

pub fn get_device_extensions() -> [&'static std::ffi::CStr; 2] {
    [ash::extensions::khr::Swapchain::name(), ash::vk::KhrMaintenance1Fn::name()]
}