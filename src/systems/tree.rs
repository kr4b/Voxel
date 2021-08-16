use specs::{Join, ReadExpect, System, WriteExpect, WriteStorage};

use winit::event::VirtualKeyCode;

use crate::components::tree::Tree;
use crate::volume::Volume;
use crate::vulkan::Vulkan;
use crate::window::keyboard::Keyboard;

pub struct TreeSystem;

impl<'a> System<'a> for TreeSystem {
    type SystemData = (
        ReadExpect<'a, Keyboard>,
        ReadExpect<'a, Vulkan>,
        WriteExpect<'a, Volume>,
        WriteStorage<'a, Tree>,
    );

    fn run(&mut self, (keyboard, vulkan, mut texture, mut trees): Self::SystemData) {
        if keyboard.pressed(VirtualKeyCode::G, None) {
            for tree in (&mut trees).join() {
                let size = texture.size();
                for _ in 0..10 {
                    tree.grow(&mut texture.data, size);
                }
                vulkan.update_texture(1, &texture.data);
            }
        }
    }
}
