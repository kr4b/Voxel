use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use noise::{Fbm, NoiseFn};

pub struct Volume {
    size: usize,
    pub data: Vec<u32>,
}

impl Volume {
    pub fn from_file<P>(path: P, size: usize) -> Self
    where
        P: AsRef<Path>,
    {
        let mut data = Vec::with_capacity(size * size * size);
        let mut file = BufReader::new(File::open(path).unwrap());

        while data.len() < size * size * size {
            let mut bytes = [0; 4];
            let mut total = 0;
            file.read_exact(&mut bytes).unwrap();
            for i in 0..4 {
                total += (bytes[i] as u32) << i * 8;
            }
            data.push(total);
        }

        Self::create_volume(size, data)
    }

    pub fn new(size: usize) -> Self {
        let data = Self::generate_data(size);
        Self::create_volume(size, data)
    }

    fn create_volume(size: usize, data: Vec<u32>) -> Self {
        Self {
            size,
            data,
        }
    }

    #[inline]
    fn to_color(r: u8, g: u8, b: u8, factor: f64) -> u32 {
        (((b as f64 * factor) as u32) << 16)
            + (((g as f64 * factor) as u32) << 8)
            + (r as f64 * factor) as u32
    }

    fn generate_data(size: usize) -> Vec<u32> {
        let mut data = Vec::with_capacity(size * size * size);
        let fbm = Fbm::new();

        for z in 0..size {
            for y in 0..size {
                for x in 0..size {
                    let noise = fbm.get([x as f64 * 0.002, y as f64 * 0.002, z as f64 * 0.002]);
                    data.push(
                        (noise - (y as f64 - size as f64 / 4.0) * 0.01 > 0.0) as u32,
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
                                (15 << 28) + Self::to_color(0b01000010, 0b01111100, 0b00111010, 1.0 - factor)
                        } else if depth < 5 {
                            data[index] =
                                (15 << 28) + Self::to_color(0b01001010, 0b01000010, 0b00110010, 1.0 - factor * 0.5)
                        } else {
                            data[index] =
                                (15 << 28) + Self::to_color(0b01000010, 0b00111100, 0b00110010, 1.0 - factor * 0.1)
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
            unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4) };
        file.write_all(slice).unwrap();

        data
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
    }
}
