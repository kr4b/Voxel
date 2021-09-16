use nalgebra::{Vector3, Vector4};

#[derive(Default)]
pub struct Light {
    pos: Vector3<f32>,
    padding: f32,
    color: Vector4<f32>,
    min_radius: f32,
    max_radius: f32,
}

impl Light {
    pub fn new(pos: Vector3<f32>, color: Vector4<f32>, min_radius: f32, max_radius: f32) -> Self {
        Self {
            pos,
            color,
            min_radius,
            max_radius,
            ..Default::default()
        }
    }
}
