use std::{ffi::CString, ptr::{null, null_mut}};

use super::LOG_MAX_LENGTH;

pub enum ShaderType {
    VERTEX,
    FRAGMENT,
}

pub struct Shader {
    pub handler: u32,
}

impl Shader {
    pub fn try_new(shader_type: ShaderType, source: &str) -> Result<Self, String> {
        let handler = match shader_type {
            ShaderType::VERTEX => unsafe { gl::CreateShader(gl::VERTEX_SHADER) },
            ShaderType::FRAGMENT => unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) },
        };

        let source = CString::new(source.as_bytes()).unwrap();
        unsafe { gl::ShaderSource(handler, 1, &source.as_ptr(), null()); }
        unsafe { gl::CompileShader(handler); }

        let mut success = gl::TRUE as i32;
        unsafe { gl::GetShaderiv(handler, gl::COMPILE_STATUS, &mut success); }
        if success != gl::TRUE as i32 {
            let mut log: Vec<u8> = Vec::with_capacity(LOG_MAX_LENGTH);
            unsafe { gl::GetShaderInfoLog(handler, LOG_MAX_LENGTH as i32, null_mut(), log.as_mut_ptr() as *mut i8); }

            let log = match std::str::from_utf8(&log) {
                Ok(s) => s,
                Err(e) => std::str::from_utf8(&log[..e.valid_up_to()]).unwrap()
            };

            Err(log.to_string())
        } else { Ok(Self { handler }) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.handler); }
    }
}

