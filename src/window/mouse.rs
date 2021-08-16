use winit::event::{ElementState, ModifiersState, MouseButton};

use super::button_resource::ButtonResource;

type Delta = (f64, f64);

#[derive(Default)]
pub struct Mouse {
    button_resource: ButtonResource<MouseButton>,
    delta: Delta,
}

impl Mouse {
    pub fn update_buttons(&mut self, button: MouseButton, state: ElementState) {
        self.button_resource.update_buttons(button, state);
    }

    pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
        self.button_resource.update_modifiers(modifiers);
    }

    pub fn update_delta(&mut self, delta: Delta) {
        self.delta = delta;
    }

    pub fn pressed(&self, button: MouseButton, modifiers: Option<ModifiersState>) -> bool {
        self.button_resource.pressed(button, modifiers)
    }

    pub fn held(&self, button: MouseButton, modifiers: Option<ModifiersState>) -> bool {
        self.button_resource.held(button, modifiers)
    }

    pub fn released(&self, button: MouseButton, modifiers: Option<ModifiersState>) -> bool {
        self.button_resource.released(button, modifiers)
    }

    pub fn delta(&self) -> Delta {
        self.delta
    }
}
