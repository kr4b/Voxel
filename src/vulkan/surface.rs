use ash::vk;

use super::instance::Instance;

pub struct Surface {
    pub loader: ash::extensions::khr::Surface,
    pub value: vk::SurfaceKHR,
}

impl Surface {
    pub fn new(
        entry: &ash::Entry,
        instance: &Instance,
        window: &winit::window::Window,
    ) -> Self {
        let value = unsafe { super::platform::create_surface(entry, &instance.value, window) }
            .expect("Failed to create surface");
        let loader = ash::extensions::khr::Surface::new(entry, &instance.value);

        Self { loader, value }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.loader.destroy_surface(self.value, None);
        }
    }
}