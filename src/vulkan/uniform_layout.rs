use std::collections::HashMap;

use ash::vk;

pub struct UniformLayout<T> {
    pub stage_flags: vk::ShaderStageFlags,
    pub uniforms: T,
}

pub type UniformLayouts<T> = HashMap<u32, UniformLayout<T>>;