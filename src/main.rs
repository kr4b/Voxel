use specs::{Builder, WorldExt};

use ash::vk;

use nalgebra::{Matrix4, Point3, Vector3};

use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

mod components;
mod dispatcher;
mod math;
mod systems;
mod volume;
mod vulkan;
mod window;

use components::tree::Tree;
use dispatcher::Dispatcher;
use math::matrices::Matrices;
use volume::*;
use vulkan::Vulkan;
use window::{keyboard::Keyboard, mouse::Mouse};

struct App {
    dispatcher: Dispatcher<'static, 'static>,
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

        let inv_proj = Self::create_inv_proj(window.inner_size());
        let view = Matrix4::identity();

        let mut dispatcher = Dispatcher::new();
        let vulkan = Vulkan::builder(window)
            .with_uniform::<Matrices>(0, vk::ShaderStageFlags::VERTEX)
            // .with_texture(1, vk::ShaderStageFlags::FRAGMENT, &texture)
            .with_dynamic_texture(1, vk::ShaderStageFlags::FRAGMENT, &texture)
            .with_uniform::<u32>(2, vk::ShaderStageFlags::FRAGMENT)
            .build();

        vulkan.update_texture(1, &texture.data);
        dispatcher.world_mut().insert(vulkan);
        dispatcher.world_mut().insert(Matrices { inv_proj, view });
        dispatcher.world_mut().insert(texture);
        dispatcher.world_mut().insert(Keyboard::default());
        dispatcher.world_mut().insert(Mouse::default());

        dispatcher.world_mut().create_entity().with(tree).build();

        Self { dispatcher }
    }

    fn create_inv_proj(size: PhysicalSize<u32>) -> Matrix4<f32> {
        Matrix4::new_perspective(
            size.width as f32 / size.height as f32,
            45.0 / 180.0 * std::f32::consts::PI,
            0.1,
            100.0,
        )
        .try_inverse()
        .unwrap()
    }

    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    let mut vulkan = self.dispatcher.world().write_resource::<Vulkan>();
                    vulkan.framebuffer_resized();
                    drop(vulkan);
                    let mut matrices = self.dispatcher.world().write_resource::<Matrices>();
                    matrices.inv_proj = Self::create_inv_proj(size);
                }
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    } => match (virtual_keycode, state) {
                        (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                            *control_flow = ControlFlow::Exit
                        }
                        (button, state) => {
                            let mut keyboard = self.dispatcher.world().write_resource::<Keyboard>();
                            keyboard.update_buttons(button, state);
                        }
                    },
                },
                WindowEvent::MouseInput { button, state, .. } => {
                    let mut mouse = self.dispatcher.world().write_resource::<Mouse>();
                    mouse.update_buttons(button, state);
                }
                WindowEvent::ModifiersChanged(state) => {
                    let mut keyboard = self.dispatcher.world().write_resource::<Keyboard>();
                    keyboard.update_modifiers(state);
                    drop(keyboard);
                    let mut mouse = self.dispatcher.world().write_resource::<Mouse>();
                    mouse.update_modifiers(state);
                }
                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    let mut mouse = self.dispatcher.world().write_resource::<Mouse>();
                    mouse.update_delta(delta);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                self.dispatcher.update();
                let mut mouse = self.dispatcher.world().write_resource::<Mouse>();
                mouse.update_delta((0.0, 0.0));
            }
            _ => (),
        });
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = window::create_window("Vulkan", 1280, 720, &event_loop);
    let app = App::new(window);
    app.run(event_loop);
}
