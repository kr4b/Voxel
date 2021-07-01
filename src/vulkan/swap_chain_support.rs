use ash::vk;

pub struct SwapChainSupport {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapChainSupport {
    pub fn new(
        surface: &super::surface::Surface,
        device: vk::PhysicalDevice,
    ) -> Self {
        let capabilities = unsafe {
            surface
                .loader
                .get_physical_device_surface_capabilities(device, surface.value)
        }
        .expect("Failed to get physical device surface capabilities");

        let formats = unsafe {
            surface
                .loader
                .get_physical_device_surface_formats(device, surface.value)
        }
        .expect("Failed to get physical device surface formats");

        let present_modes = unsafe {
            surface
                .loader
                .get_physical_device_surface_present_modes(device, surface.value)
        }
        .expect("Failed to get physical device surface present modes");

        Self {
            capabilities,
            formats,
            present_modes,
        }
    }
}