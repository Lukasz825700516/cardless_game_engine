use std::mem::size_of;

use super::{vertex_attribute::Vertex, buffer::{Buffer, BufferType}, texture::Texture};

pub struct Batch<T>
where T: Vertex {
    pub vbo_capacity: usize,
    pub vbo: Buffer<T>,

    pub ebo_capacity: usize,
    pub ebo: Buffer<u32>,

    pub textures_capacity: usize,
    pub textures: Vec<u32>,
}


impl<T> Batch<T>
where T: Vertex {
    pub fn new() -> Self {
        let textures_capacity = 16;
        Self {
            vbo_capacity: 2048,
            vbo: Buffer::new(BufferType::VERTEX, Vec::new(), size_of::<T>()),
            ebo_capacity: 2048,
            ebo: Buffer::new(BufferType::ELEMENT, Vec::new(), size_of::<u32>()),
            textures_capacity,
            textures: Vec::with_capacity(textures_capacity),
        }
    }

    pub fn get_texture_slot(&mut self, texture: &Texture) -> Option<i32> {
        match self.textures.iter().position(|&id| id == texture.handler) {
            Some(slot) => Some(slot as i32),
            None => {
                let next_slot = self.textures.len();
                if next_slot < self.textures_capacity {
                    self.textures.push(texture.handler);
                    Some(next_slot as i32)
                } else {
                    None
                }
            }
        }
    }
}

