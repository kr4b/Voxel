use nalgebra::{Matrix4, Point3, Rotation3, Vector3};

mod branch;
mod colors;
mod render;
mod shader;
mod vao;
mod volume;
mod window;

use branch::*;
use render::*;
use shader::*;
use vao::*;
use volume::*;
use window::*;

struct App {
    vao: VertexArray,
    shader: Shader,
    texture: Volume,
    inv_proj: Matrix4<f32>,
    view: Matrix4<f32>,
    pitch: f32,
    yaw: f32,
    speed: f32,
    root: Branch,
    sequence: Sequence,
}

impl App {
    pub fn new() -> Self {
        let texture = Volume::from_file("assets/world", 512);

        let size = texture.size();
        let mut height = 0;

        for y in 0..size {
            if texture.data[(size / 2) * size * size + y * size + size / 2] > 0 {
                height = y;
            }
        }

        Self {
            vao: VertexArray::new(),
            shader: Shader::new(
                include_bytes!("../shaders/voxel.vert"),
                include_bytes!("../shaders/voxel.frag"),
            )
            .unwrap(),
            texture,
            // texture: Volume::new(512),
            inv_proj: Matrix4::new_perspective(
                16.0 / 9.0,
                70.0 / 180.0 * std::f32::consts::PI,
                0.1,
                100.0,
            )
            .try_inverse()
            .unwrap(),
            view: Matrix4::new_translation(&Vector3::new(0.0, -90.0, 30.0)),
            pitch: 0.0,
            yaw: 0.0,
            speed: 5.0,
            root: Branch::new(
                Point3::new(size as f32 / 2.0, height as f32, size as f32 / 2.0),
                15.0,
                4,
                Rotation3::identity(),
            ),
            sequence: Sequence::Branch(
                (6, 20.0, 0.5, 4, 0),
                // Bottom branches
                Box::new(Sequence::Branch(
                    (3, 8.0, 8.0, 1, 60),
                    Box::new(Sequence::Leaf),
                    Box::new(Sequence::Leaf),
                )),
                // Second center
                Box::new(Sequence::Branch(
                    (6, 27.5, 12.5, 3, 30),
                    // Second branches
                    Box::new(Sequence::Leaf),
                    // Tertairy center
                    Box::new(Sequence::Branch(
                        (6, 15.0, 11.0, 2, 30),
                        // Tertairy branches
                        Box::new(Sequence::Branch(
                            (3, 5.0, 0.0, 1, 0),
                            Box::new(Sequence::Leaf),
                            Box::new(Sequence::Leaf),
                        )),
                        // Quarternary center
                        Box::new(Sequence::Branch(
                            (6, 10.0, 5.0, 1, 30),
                            Box::new(Sequence::Leaf),
                            // Top
                            Box::new(Sequence::Branch(
                                (4, 2.0, 2.0, 1, 0),
                                Box::new(Sequence::Leaf),
                                Box::new(Sequence::Leaf),
                            )),
                        )),
                    )),
                )),
            ),
        }
    }

    fn grow(&mut self) {
        let size = self.texture.size();
        self.root
            .extend(&self.sequence, &mut self.texture.data, size);
        self.texture.sub();
    }
}

impl Render for App {
    fn render(&self) {
        self.shader.bind();
        self.vao.bind();
        self.texture.bind();

        let rotation = Matrix4::<f32>::from_euler_angles(self.yaw, self.pitch, 0.0);
        let view = self.view * rotation;

        unsafe {
            gl::UniformMatrix4fv(
                self.shader.uniform_location("inv_proj"),
                1,
                gl::FALSE,
                self.inv_proj.data.as_ptr(),
            );
            gl::UniformMatrix4fv(
                self.shader.uniform_location("view"),
                1,
                gl::FALSE,
                view.data.as_ptr(),
            );
            gl::Uniform1ui(
                self.shader.uniform_location("size"),
                self.texture.size() as _,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn update(&mut self, key: &str, delta: (f64, f64)) {
        let rotation = Matrix4::<f32>::from_euler_angles(self.yaw, self.pitch, 0.0);

        let dir = Vector3::new(rotation[8], rotation[9], rotation[10]);
        let up = Vector3::new(rotation[4], rotation[5], rotation[6]);
        let left = Vector3::new(rotation[0], rotation[1], rotation[2]);

        let translation = match key {
            "W" => -self.speed * dir,
            "S" => self.speed * dir,
            "A" => -self.speed * left,
            "D" => self.speed * left,
            "SPACE" => self.speed * up,
            "SHIFT" => -self.speed * up,
            _ => Vector3::new(0.0, 0.0, 0.0),
        };

        match key {
            "UP" => self.speed *= 2.0,
            "DOWN" => self.speed /= 2.0,
            "G" => self.grow(),
            _ => (),
        }

        self.view = self.view.append_translation(&translation);

        let scalar = 1.0 / 100.0;
        self.pitch -= delta.0 as f32 * scalar;
        self.yaw -= delta.1 as f32 * scalar;
    }
}

fn main() {
    let window = Window::new(1280, 720, "Voxels").unwrap();
    let app = App::new();
    window.run(Box::new(app));
}
