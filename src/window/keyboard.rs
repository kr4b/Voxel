use std::collections::HashSet;
use winit::event::{ElementState, ModifiersState, VirtualKeyCode};

#[derive(Default)]
pub struct Keyboard {
    pressed: HashSet<VirtualKeyCode>,
    held: HashSet<VirtualKeyCode>,
    released: HashSet<VirtualKeyCode>,
    modifiers: ModifiersState,
}

impl Keyboard {
    pub fn update_keys(&mut self, virtual_keycode: Option<VirtualKeyCode>, state: ElementState) {
        if let Some(virtual_keycode) = virtual_keycode {
            match state {
                ElementState::Pressed if self.pressed.contains(&virtual_keycode) => {
                    self.pressed.remove(&virtual_keycode);
                    self.held.insert(virtual_keycode);
                },
                ElementState::Pressed => {
                    self.pressed.insert(virtual_keycode);
                },
                ElementState::Released => {
                    self.held.remove(&virtual_keycode);
                    self.released.insert(virtual_keycode);
                }
            }
        }
    }

    pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    pub fn pressed(&self, virtual_keycode: VirtualKeyCode, modifiers: Option<ModifiersState>) -> bool {
        self.pressed.contains(&virtual_keycode) && self.check_modifiers(modifiers)
    }

    pub fn held(&self, virtual_keycode: VirtualKeyCode, modifiers: Option<ModifiersState>) -> bool {
        self.held.contains(&virtual_keycode) && self.check_modifiers(modifiers)
    }

    pub fn released(&self, virtual_keycode: VirtualKeyCode, modifiers: Option<ModifiersState>) -> bool {
        self.released.contains(&virtual_keycode) && self.check_modifiers(modifiers)
    }

    fn check_modifiers(&self, modifiers: Option<ModifiersState>) -> bool {
        match modifiers {
            Some(modifiers) if modifiers == self.modifiers => true,
            _ => false,
        }
    }
}