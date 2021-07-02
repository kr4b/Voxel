use winit::event::{ElementState, ModifiersState, VirtualKeyCode};

use super::button_resource::ButtonResource;

pub struct Keyboard {
    button_resource: ButtonResource<VirtualKeyCode>,
}

impl Keyboard {
    pub fn update_keys(&mut self, button: Option<VirtualKeyCode>, state: ElementState) {
        self.button_resource.update_keys(button, state);
    }

    pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
        self.button_resource.update_modifiers(modifiers);
    }

    pub fn pressed(&self, button: VirtualKeyCode, modifiers: Option<ModifiersState>) -> bool {
        self.button_resource.pressed(button, modifiers)
    }

    pub fn held(&self, button: VirtualKeyCode, modifiers: Option<ModifiersState>) -> bool {
        self.button_resource.held(button, modifiers)
    }

    pub fn released(&self, button: VirtualKeyCode, modifiers: Option<ModifiersState>) -> bool {
        self.button_resource.released(button, modifiers)
    }
}