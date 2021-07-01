use std::rc::Rc;

use ash::vk;

use nalgebra::{Matrix4, Point3, Vector3};
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod math;
mod tree;
mod volume;
mod vulkan;

use math::matrices::Matrices;
use tree::*;
use volume::*;
use vulkan::*;

struct App {
    vulkan: Vulkan,
    window: Rc<Window>,
    texture: Volume,
    inv_proj: Matrix4<f32>,
    view: Matrix4<f32>,
    pitch: f32,
    yaw: f32,
    speed: f32,
    key: char,
    delta: (f64, f64),
    tree: Tree,
}

impl App {
    pub fn new(window: Window) -> Self {
        let mut texture = Volume::from_file("assets/world", 512);
        // let mut texture = Volume::new(512);

        let size = texture.size();
        let mut height = 0;

        for y in 0..size {
            if texture.data[(size / 2) * size * size + y * size + size / 2] > 0 {
                height = y;
            }
        }

        let start = Point3::new(size as f32 / 2.0, height as f32, size as f32 / 2.0);
        let offset = Vector3::new(
            rand::random::<f32>() * 10.0 - 5.0,
            35.0 + rand::random::<f32>() * 10.0,
            rand::random::<f32>() * 10.0 - 5.0,
        );
        let tree = Tree::new(start + offset, 30.0, 400, start, &mut texture.data, size);

        let inv_proj = Self::create_inv_proj(&window);

        let window = Rc::new(window);
        let mut vulkan = Vulkan::builder(Rc::clone(&window))
            .with_uniform::<Matrices>(0, vk::ShaderStageFlags::VERTEX)
            // .with_texture(1, vk::ShaderStageFlags::FRAGMENT, &texture)
            .with_dynamic_texture(1, vk::ShaderStageFlags::FRAGMENT, &texture)
            .with_uniform::<u32>(2, vk::ShaderStageFlags::FRAGMENT)
            .build();

        vulkan.update_texture(1, &texture.data);
        Self {
            vulkan,
            window,
            texture,
            inv_proj,
            view: Matrix4::new_translation(&Vector3::new(0.0, 0.0, 30.0)),
            pitch: 0.0,
            yaw: 0.0,
            speed: 1.0,
            key: ' ',
            delta: (0.0, 0.0),
            tree,
        }
    }

    fn create_inv_proj(window: &Window) -> Matrix4<f32> {
        let window_size = window.inner_size().to_logical::<f32>(window.scale_factor());
        Matrix4::new_perspective(
            window_size.width / window_size.height,
            45.0 / 180.0 * std::f32::consts::PI,
            0.1,
            100.0,
        )
        .try_inverse()
        .unwrap()
    }

    fn update(&mut self) {
        let rotation = Matrix4::<f32>::from_euler_angles(self.yaw, self.pitch, 0.0);

        let dir = Vector3::new(rotation[8], rotation[9], rotation[10]);
        let up = Vector3::new(rotation[4], rotation[5], rotation[6]);
        let left = Vector3::new(rotation[0], rotation[1], rotation[2]);
        let translation = match self.key {
            'W' => -self.speed * dir,
            'S' => self.speed * dir,
            'A' => -self.speed * left,
            'D' => self.speed * left,
            'Q' => self.speed * up,
            'E' => -self.speed * up,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };
        if self.key == 'G' {
            let size = self.texture.size();
            for _ in 0..10 {
                self.tree.grow(&mut self.texture.data, size);
            }
            self.vulkan.update_texture(1, &self.texture.data);
        }
        self.key = ' ';
        self.view = self.view.append_translation(&translation);

        let scalar = 1.0 / 100.0;
        self.pitch -= self.delta.0 as f32 * scalar;
        self.yaw -= self.delta.1 as f32 * scalar;
        self.delta = (0.0, 0.0);

        let view = self.view * rotation;
        let matrices = Matrices {
            view,
            inv_proj: self.inv_proj,
        };
        self.vulkan.begin_draw();
        self.vulkan.update_uniform(0, matrices);
        self.vulkan.update_uniform(2, self.texture.size() as u32);
        self.vulkan.end_draw();
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(_) => {
                    self.vulkan.framebuffer_resized();
                    self.inv_proj = Self::create_inv_proj(self.window.as_ref());
                },
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            *control_flow = ControlFlow::Exit
                        }
                        (Some(VirtualKeyCode::W), ElementState::Pressed) => {
                            self.key = 'W';
                        }
                        (Some(VirtualKeyCode::A), ElementState::Pressed) => {
                            self.key = 'A';
                        }
                        (Some(VirtualKeyCode::S), ElementState::Pressed) => {
                            self.key = 'S';
                        }
                        (Some(VirtualKeyCode::D), ElementState::Pressed) => {
                            self.key = 'D';
                        }
                        (Some(VirtualKeyCode::Q), ElementState::Pressed) => {
                            self.key = 'Q';
                        }
                        (Some(VirtualKeyCode::E), ElementState::Pressed) => {
                            self.key = 'E';
                        }
                        (Some(VirtualKeyCode::G), ElementState::Pressed) => {
                            self.key = 'G';
                        }
                        _ => {}
                    },
                },
                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    self.delta = delta;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                self.update();
            }
            _ => (),
        });
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = window::create_window(&event_loop);
    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);
    let app = App::new(window);
    app.run(event_loop);
}
