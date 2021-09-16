use std::collections::HashMap;

use ash::vk;

pub struct BufferLayout<T> {
    pub stage_flags: vk::ShaderStageFlags,
    pub buffer: T,
}

pub type BufferLayouts<T> = HashMap<u32, BufferLayout<T>>;