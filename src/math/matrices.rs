use nalgebra::Matrix4;

#[derive(Clone, Debug)]
pub struct Matrices {
    pub inv_proj: Matrix4<f32>,
    pub view: Matrix4<f32>,
}