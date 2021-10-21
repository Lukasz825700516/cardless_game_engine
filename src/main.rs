extern crate glfw;
use std::{borrow::BorrowMut, ffi::{CStr, CString, c_void}, mem::size_of, ptr::{null, null_mut}, slice};

use gl::types::{GLchar, GLint};

use self::glfw::Context;

extern crate gl;

const LOG_MAX_LENGTH: usize = 512;

enum ShaderType {
    VERTEX,
    FRAGMENT,
}
struct Shader {
    handler: u32,
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

        let mut success = gl::TRUE as GLint;
        unsafe { gl::GetShaderiv(handler, gl::COMPILE_STATUS, &mut success); }
        if success != gl::TRUE as GLint {
            let mut log: Vec<u8> = Vec::with_capacity(LOG_MAX_LENGTH);
            unsafe { gl::GetShaderInfoLog(handler, LOG_MAX_LENGTH as i32, null_mut(), log.as_mut_ptr() as *mut GLchar); }

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

struct ShaderProgram {
    handler: u32,
}

impl ShaderProgram {
    pub fn new() -> Self {
        let handler = unsafe { gl::CreateProgram() };

        Self { handler }
    }

    pub fn add(&mut self, shader: Shader) {
        unsafe { gl::AttachShader(self.handler, shader.handler); }
    }

    pub fn link(&mut self) -> Result<(), String> {
        unsafe { gl::LinkProgram(self.handler); }

        let mut success = gl::TRUE as GLint;
        unsafe { gl::GetProgramiv(self.handler, gl::LINK_STATUS, &mut success); }
        if success != gl::TRUE as GLint {
            let mut log: Vec<u8> = Vec::with_capacity(LOG_MAX_LENGTH);
            unsafe { log.set_len(LOG_MAX_LENGTH); }
            unsafe { gl::GetProgramInfoLog(self.handler, LOG_MAX_LENGTH as i32, null_mut(), log.as_mut_ptr() as *mut GLchar); }

            let log = match std::str::from_utf8(&log) {
                Ok(s) => s,
                Err(e) => std::str::from_utf8(&log[..e.valid_up_to()]).unwrap()
            };

            Err(log.to_string())
        } else {
            Ok(())
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.handler); }
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize glfw");
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, _events) = glfw.create_window(800, 600, "Hello world", glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol));

    let vertex_buffer: Vec<f32> = vec![
        0., 1., 0.,
        1., -0.5, 0.,
        -1., -0.5, 0.,
    ];
    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe { gl::BindVertexArray(vao); }

    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo); }
    unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo); }
    unsafe { gl::BufferData(gl::ARRAY_BUFFER, (vertex_buffer.len() * size_of::<f32>()) as isize, vertex_buffer.as_ptr() as *const c_void, gl::STATIC_DRAW); }

    let vertex_shader = "
#version 330 core
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
        ";
    let vertex_shader = Shader::try_new(ShaderType::VERTEX, vertex_shader).unwrap();

    let fragment_shader = "
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
    ";

    let fragment_shader = Shader::try_new(ShaderType::FRAGMENT, fragment_shader).unwrap();

    let mut shader_program = ShaderProgram::new();
    shader_program.add(vertex_shader);
    shader_program.add(fragment_shader);
    shader_program.link().unwrap();

    unsafe { gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as i32, null()); }
    unsafe { gl::EnableVertexAttribArray(0); }

    unsafe { gl::UseProgram(shader_program.handler); }


    while !window.should_close() {
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        window.swap_buffers();
        glfw.poll_events();
    }
}
