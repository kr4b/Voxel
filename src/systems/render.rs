use specs::{Read, ReadExpect, System, WriteExpect};

use crate::math::matrices::Matrices;
use crate::volume::Volume;
use crate::vulkan::Vulkan;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        WriteExpect<'a, Vulkan>,
        Read<'a, Matrices>,
        ReadExpect<'a, Volume>,
    );

    fn run(&mut self, (mut vulkan, matrices, texture): Self::SystemData) {
        let matrices = matrices.clone();
        vulkan.begin_draw();
        vulkan.update_buffer(0, matrices);
        vulkan.update_buffer(2, texture.size() as u32);
        vulkan.end_draw();
    }
}
