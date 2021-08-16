use specs::prelude::*;

use crate::systems::{CameraSystem, RenderSystem, TreeSystem};

pub struct Dispatcher<'a, 'b> {
    value: specs::Dispatcher<'a, 'b>,
    world: World,
}

impl Dispatcher<'_, '_> {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut value = DispatcherBuilder::new()
            .with(CameraSystem::new(), "camera", &[])
            .with(TreeSystem, "tree", &[])
            .with_thread_local(RenderSystem)
            .build();

        value.setup(&mut world);

        Self {
            value,
            world,
        }
    }

    pub fn update(&mut self) {
        self.value.dispatch(&mut self.world);
        self.world.maintain();
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
}