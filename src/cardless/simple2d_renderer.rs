use std::{mem::size_of, ptr::null};

use glm::vec2;
use memoffset::offset_of;

use crate::cardless::vertex_attribute::{VertexAttribute, VertexAttributeType};

use super::{vertex_attribute::Vertex, shader_program::ShaderProgram, batch::Batch, shader::{Shader, ShaderType}, texture::Texture};

#[repr(C)]
pub struct Simple2DVertex {
    pub pos: glm::Vec2,
    pub uv: glm::Vec2,
    pub texture: i32,
}

impl Vertex for Simple2DVertex {
    fn get_attributes_layout() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::F32, 2, false, size_of::<Self>(), offset_of!(Self, pos)),
            VertexAttribute::new(VertexAttributeType::F32, 2, false, size_of::<Self>(), offset_of!(Self, uv)),
            VertexAttribute::new(VertexAttributeType::I32, 1, false, size_of::<Self>(), offset_of!(Self, texture)),
        ]
    }
}

pub struct BatchRenderer {
    shader: ShaderProgram,
    batch: Batch<Simple2DVertex>,
}

impl BatchRenderer {
    pub fn new(fragment: &str, vertex: &str) -> Self {
        let batch = Batch::new();
        let fragment = Shader::try_new(ShaderType::FRAGMENT, fragment).unwrap();
        let vertex = Shader::try_new(ShaderType::VERTEX, vertex).unwrap();
        let shader = ShaderProgram::try_new(vertex, fragment, &Simple2DVertex::get_attributes_layout()).unwrap();

        Self {
            shader,
            batch,
        }
    }

    pub fn bind(&mut self) {
        self.shader.activate();
        self.shader.set_1iv("u_texture", &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    }

    pub fn push_square(&mut self, pos: glm::Vec2, size: glm::Vec2) {
        if self.batch.vbo.data.len() + 4 > self.batch.vbo_capacity 
        || self.batch.ebo.data.len() + 6 > self.batch.ebo_capacity {
            self.flush()
        }

        let first_vertex = self.batch.vbo.data.len() as u32;

        self.batch.vbo.data.push(Simple2DVertex { pos: pos + 0., uv: glm::vec2(0., 0.), texture: 0});
        self.batch.vbo.data.push(Simple2DVertex { pos: pos + vec2(size.x, 0.), uv: glm::vec2(1., 0.), texture: 0});
        self.batch.vbo.data.push(Simple2DVertex { pos: pos + vec2(0., size.y), uv: glm::vec2(0., 1.), texture: 0});
        self.batch.vbo.data.push(Simple2DVertex { pos: pos + size, uv: glm::vec2(1., 1.), texture: 0});

        self.batch.ebo.data.push(first_vertex + 0);
        self.batch.ebo.data.push(first_vertex + 1);
        self.batch.ebo.data.push(first_vertex + 2);
        self.batch.ebo.data.push(first_vertex + 2);
        self.batch.ebo.data.push(first_vertex + 1);
        self.batch.ebo.data.push(first_vertex + 3);

    }

    pub fn push_square_texture(&mut self, pos: glm::Vec2, size: glm::Vec2, texture: &Texture) {
        if self.batch.vbo.data.len() + 4 > self.batch.vbo_capacity 
        || self.batch.ebo.data.len() + 6 > self.batch.ebo_capacity {
            self.flush()
        }

        let texture = match self.batch.get_texture_slot(texture) {
            Some(slot) => slot,
            None => {
                self.flush();
                self.batch.get_texture_slot(texture).unwrap()
            }
        };

        let first_vertex = self.batch.vbo.data.len() as u32;

        self.batch.vbo.data.push(Simple2DVertex { pos: pos + 0., uv: glm::vec2(0., 0.), texture});
        self.batch.vbo.data.push(Simple2DVertex { pos: pos + vec2(size.x, 0.), uv: glm::vec2(1., 0.), texture});
        self.batch.vbo.data.push(Simple2DVertex { pos: pos + vec2(0., size.y), uv: glm::vec2(0., 1.), texture});
        self.batch.vbo.data.push(Simple2DVertex { pos: pos + size, uv: glm::vec2(1., 1.), texture});

        self.batch.ebo.data.push(first_vertex + 0);
        self.batch.ebo.data.push(first_vertex + 1);
        self.batch.ebo.data.push(first_vertex + 2);
        self.batch.ebo.data.push(first_vertex + 2);
        self.batch.ebo.data.push(first_vertex + 1);
        self.batch.ebo.data.push(first_vertex + 3);
    }

    pub fn flush(&mut self) {
        self.batch.vbo.flush();
        self.batch.ebo.flush();

        for (slot, &texture) in self.batch.textures.iter().enumerate() {
            unsafe { gl::ActiveTexture(gl::TEXTURE0 + slot as u32); }
            unsafe { gl::BindTexture(gl::TEXTURE_2D, texture); }
        }

        unsafe { gl::DrawElements(gl::TRIANGLES, self.batch.ebo.data.len() as i32, gl::UNSIGNED_INT, null()); }

        if self.batch.textures.len() == self.batch.textures_capacity {
            self.batch.textures.clear();
        }
        self.batch.vbo.data.clear();
        self.batch.ebo.data.clear();
    }
}

