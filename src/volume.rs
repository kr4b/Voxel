use gl::types::GLuint;
use noise::{Fbm, MultiFractal, NoiseFn};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub struct Volume {
    size: usize,
    pub data: Vec<u16>,
    texture: GLuint,
}

impl Volume {
    pub fn from_file<P>(path: P, size: usize) -> Self
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::with_capacity(size * size * size);
        let mut file = BufReader::new(File::open(path).unwrap());

        while data.len() < size * size * size {
            let mut bytes = [0; 2];
            file.read_exact(&mut bytes).unwrap();
            data.push(((bytes[1] as u16) << 8) | bytes[0] as u16);
        }

        Self::create_volume(size, data)
    }

    pub fn new(size: usize) -> Self {
        let data = Self::generate_data(size);
        Self::create_volume(size, data)
    }

    fn create_volume(size: usize, data: Vec<u16>) -> Self {
        let mut texture = 0;
        unsafe {
            gl::CreateTextures(gl::TEXTURE_3D, 1, &mut texture);
            gl::BindTexture(gl::TEXTURE_3D, texture);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                gl::R16UI as _,
                size as _,
                size as _,
                size as _,
                0,
                gl::RED_INTEGER,
                gl::UNSIGNED_SHORT,
                data.as_ptr() as *const _,
            );
        }

        Self {
            size,
            data,
            texture,
        }
    }

    #[inline]
    fn to_color(r: u16, g: u16, b: u16, factor: f64) -> u16 {
        (((r as f64 * factor) as u16) << 10)
            + (((g as f64 * factor) as u16) << 5)
            + (b as f64 * factor) as u16
    }

    fn generate_data(size: usize) -> Vec<u16> {
        let mut data = Vec::with_capacity(size * size * size);
        let fbm = Fbm::new();

        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let noise = fbm.get([x as f64 * 0.002, y as f64 * 0.002, z as f64 * 0.002]);
                    data.push(
                        (noise - (y as f64 - size as f64 / 4.0) * 0.01 > 0.0) as u16 * (1 << 15),
                    );
                }
            }
        }

        for z in 0..size {
            for x in 0..size {
                let mut depth = 0;
                for y in 0..size {
                    let index = size * size * z + size * (size - y - 1) + x;
                    if data[index] > 0 {
                        let factor = fbm.get([x as f64, y as f64, z as f64]);
                        if depth == 0 {
                            data[index] =
                                (1 << 15) + Self::to_color(0b01000, 0b01111, 0b00111, 1.0 - factor)
                        } else if depth < 5 {
                            data[index] =
                                (1 << 15) + Self::to_color(0b01001, 0b01000, 0b00110, 1.0 - factor * 0.5)
                        } else {
                            data[index] =
                                (1 << 15) + Self::to_color(0b01000, 0b00111, 0b00110, 1.0 - factor * 0.1)
                        }

                        depth += 1;
                    } else {
                        depth = 0;
                    }
                }
            }
        }

        let mut file = std::fs::File::create("assets/world").unwrap();
        let slice =
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2) };
        file.write_all(slice).unwrap();

        data
    }

    pub fn sub(&self) {
        self.bind();
        unsafe {
            gl::TexSubImage3D(
                gl::TEXTURE_3D,
                0,
                0,
                0,
                0,
                self.size as _,
                self.size as _,
                self.size as _,
                gl::RED_INTEGER,
                gl::UNSIGNED_SHORT,
                self.data.as_ptr() as *const _,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_3D, self.texture);
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture);
        }
    }
}
