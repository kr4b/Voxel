use std::collections::HashSet;
use winit::event::{ElementState, ModifiersState};

pub struct ButtonResource<T> where T: std::cmp::Eq + std::hash::Hash {
    pressed: HashSet<T>,
    held: HashSet<T>,
    released: HashSet<T>,
    modifiers: ModifiersState,
}

impl<T> ButtonResource<T> where T: std::cmp::Eq + std::hash::Hash {
    pub fn update_buttons(&mut self, button: T, state: ElementState) {
        match state {
            ElementState::Pressed if self.pressed.contains(&button) => {
                self.pressed.remove(&button);
                self.held.insert(button);
            },
            ElementState::Pressed => {
                self.pressed.insert(button);
            },
            ElementState::Released => {
                self.pressed.remove(&button);
                self.held.remove(&button);
                self.released.insert(button);
            }
        }
    }

    pub fn update_modifiers(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    pub fn pressed(&self, button: T, modifiers: Option<ModifiersState>) -> bool {
        self.pressed.contains(&button) && self.check_modifiers(modifiers)
    }

    pub fn held(&self, button: T, modifiers: Option<ModifiersState>) -> bool {
        self.held.contains(&button) && self.check_modifiers(modifiers)
    }

    pub fn released(&self, button: T, modifiers: Option<ModifiersState>) -> bool {
        self.released.contains(&button) && self.check_modifiers(modifiers)
    }

    fn check_modifiers(&self, modifiers: Option<ModifiersState>) -> bool {
        match modifiers {
            Some(modifiers) if modifiers == self.modifiers => true,
            _ => self.modifiers.is_empty(),
        }
    }
}

impl<T> Default for ButtonResource<T> where T: std::cmp::Eq + std::hash::Hash {
    fn default() -> Self {
        Self {
            pressed: HashSet::new(),
            held: HashSet::new(),
            released: HashSet::new(),
            modifiers: ModifiersState::default(),
        }
    }
}
