use nalgebra::Matrix4;

#[derive(Clone, Debug, Default)]
pub struct Matrices {
    pub inv_proj: Matrix4<f32>,
    pub view: Matrix4<f32>,
}