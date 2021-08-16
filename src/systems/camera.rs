use nalgebra::{Matrix4, Vector3};

use specs::{ReadExpect, System, Write};

use winit::event::VirtualKeyCode;

use crate::math::matrices::Matrices;
use crate::window::{keyboard::Keyboard, mouse::Mouse};

pub struct CameraSystem {
    view: Matrix4<f32>,
    pitch: f32,
    yaw: f32,
}

impl CameraSystem {
    pub fn new() -> Self {
        Self {
            view: Matrix4::new_translation(&Vector3::new(0.0, 0.0, 0.0)),
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (ReadExpect<'a, Keyboard>, ReadExpect<'a, Mouse>, Write<'a, Matrices>);

    fn run(&mut self, (keyboard, mouse, mut matrices): Self::SystemData) {
        let rotation = Matrix4::<f32>::from_euler_angles(self.yaw, self.pitch, 0.0);

        let forward = Vector3::new(rotation[8], rotation[9], rotation[10]);
        let up = Vector3::new(rotation[4], rotation[5], rotation[6]);
        let left = Vector3::new(rotation[0], rotation[1], rotation[2]);

        let translation = if keyboard.held(VirtualKeyCode::W, None) {
            -forward
        } else if keyboard.held(VirtualKeyCode::S, None) {
            forward
        } else if keyboard.held(VirtualKeyCode::A, None) {
            -left
        } else if keyboard.held(VirtualKeyCode::D, None) {
            left
        } else if keyboard.held(VirtualKeyCode::Q, None) {
            -up
        } else if keyboard.held(VirtualKeyCode::E, None) {
            up
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        };

        self.view = self.view.append_translation(&translation);

        let scalar = 1.0 / 100.0;
        self.pitch -= mouse.delta().0 as f32 * scalar;
        self.yaw -= mouse.delta().1 as f32 * scalar;

        matrices.view = self.view * rotation;
    }
}