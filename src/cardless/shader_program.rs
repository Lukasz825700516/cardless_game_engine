use std::{collections::HashMap, ffi::{CString, c_void}, ptr::null_mut};

use super::{LOG_MAX_LENGTH, shader::Shader, vertex_attribute::{VertexAttribute, VertexAttributeType}};

pub struct ShaderProgram {
    pub handler: u32,
    pub uniforms: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn try_new(vertex: Shader, fragment: Shader, layout: &[VertexAttribute]) -> Result<Self, String> {
        let handler = unsafe { gl::CreateProgram() };
        unsafe { gl::AttachShader(handler, vertex.handler); }
        unsafe { gl::AttachShader(handler, fragment.handler); }


        unsafe { gl::LinkProgram(handler); }

        let mut success = gl::TRUE as i32;
        unsafe { gl::GetProgramiv(handler, gl::LINK_STATUS, &mut success); }
        if success != gl::TRUE as i32 {
            let mut log: Vec<u8> = Vec::with_capacity(LOG_MAX_LENGTH);
            unsafe { log.set_len(LOG_MAX_LENGTH); }
            unsafe { gl::GetProgramInfoLog(handler, LOG_MAX_LENGTH as i32, null_mut(), log.as_mut_ptr() as *mut i8); }

            let log = match std::str::from_utf8(&log) {
                Ok(s) => s,
                Err(e) => std::str::from_utf8(&log[..e.valid_up_to()]).unwrap()
            };

            Err(log.to_string())
        } else {
            for (i, attribute) in layout.iter().enumerate() {
                let normalized = match attribute.normalized {
                    true => gl::TRUE,
                    false => gl::FALSE,
                };
                match attribute.attribute_type {
                    VertexAttributeType::F32 => {
                        let attribute_type = gl::FLOAT;
                        unsafe {
                            gl::VertexAttribPointer(
                                i as u32,
                                attribute.size as i32,
                                attribute_type,
                                normalized,
                                attribute.stride as i32,
                                attribute.width as *const c_void,
                            );
                        }
                    }
                    VertexAttributeType::I32 => {
                        let attribute_type = gl::INT;
                        unsafe {
                            gl::VertexAttribIPointer(
                                i as u32,
                                attribute.size as i32,
                                attribute_type,
                                attribute.stride as i32,
                                attribute.width as *const c_void,
                            );
                        }
                    }
                    VertexAttributeType::U32 => {
                        let attribute_type = gl::UNSIGNED_INT;
                        unsafe {
                            gl::VertexAttribIPointer(
                                i as u32,
                                attribute.size as i32,
                                attribute_type,
                                attribute.stride as i32,
                                attribute.width as *const c_void,
                            );
                        }
                    }
                };

                unsafe { gl::EnableVertexAttribArray(i as u32); }
            }

            Ok(Self {
                handler,
                uniforms: HashMap::new()
            })
        }
    }

    fn getload_uniform(&mut self, name: &str) -> Option<i32> {
        match self.uniforms.get(name) {
            Some(v) => Some(v.to_owned()),
            None => {
                let c_name = CString::new(name).unwrap();
                match unsafe { gl::GetUniformLocation(self.handler, c_name.as_ptr()) } {
                    -1 => None,
                    uniform => {
                        self.uniforms.insert(name.to_string(), uniform);
                        Some(uniform)
                    }
                }
            }
        }
    }

    pub fn set_1iv(&mut self, name: &str, v: &[i32]) {
        match self.getload_uniform(name) {
            Some(uniform) => unsafe {gl::Uniform1iv(uniform, v.len() as i32, v.as_ptr()); },
            None => {}
        }
    }

    pub fn set_1uv(&mut self, name: &str, v: &[u32]) {
        match self.getload_uniform(name) {
            Some(uniform) => unsafe {gl::Uniform1uiv(uniform, v.len() as i32, v.as_ptr()); },
            None => {}
        }
    }

    pub fn set_1i(&mut self, name: &str, v0: i32) {
        match self.getload_uniform(name) {
            Some(uniform) => unsafe {gl::Uniform1i(uniform, v0); },
            None => {}
        }
    }

    pub fn set_3f32(&mut self, name: &str, v0: f32, v1: f32, v2: f32) {
        match self.getload_uniform(name) {
            Some(uniform) => unsafe {gl::Uniform3f(uniform, v0, v1, v2); },
            None => {}
        }
    }

    pub fn activate(&mut self) -> &mut Self {
        unsafe { gl::UseProgram(self.handler); }

        self
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handler); }
    }
}

