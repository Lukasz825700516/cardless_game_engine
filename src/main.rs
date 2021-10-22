extern crate glfw;
use std::{borrow::BorrowMut, collections::HashMap, ffi::{CStr, CString, c_void}, mem::size_of, ptr::{null, null_mut}, slice, time::Duration};

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
    uniforms: HashMap<String, i32>,
}

impl ShaderProgram {
    pub fn try_new(vertex: Shader, fragment: Shader, layout: &[VertexAttribute]) -> Result<Self, String> {
        let handler = unsafe { gl::CreateProgram() };
        unsafe { gl::AttachShader(handler, vertex.handler); }
        unsafe { gl::AttachShader(handler, fragment.handler); }


        unsafe { gl::LinkProgram(handler); }

        let mut success = gl::TRUE as GLint;
        unsafe { gl::GetProgramiv(handler, gl::LINK_STATUS, &mut success); }
        if success != gl::TRUE as GLint {
            let mut log: Vec<u8> = Vec::with_capacity(LOG_MAX_LENGTH);
            unsafe { log.set_len(LOG_MAX_LENGTH); }
            unsafe { gl::GetProgramInfoLog(handler, LOG_MAX_LENGTH as i32, null_mut(), log.as_mut_ptr() as *mut GLchar); }

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
                let attribute_type = match attribute.attribute_type {
                    VertexAttributeType::F32 => gl::FLOAT,
                    VertexAttributeType::I32 => gl::INT,
                    VertexAttributeType::U32 => gl::UNSIGNED_INT,
                };
                unsafe {
                    gl::VertexAttribPointer(
                        i as u32,
                        attribute.size as i32,
                        attribute_type as u32,
                        normalized as u8,
                        attribute.stride as i32,
                        attribute.width as *const c_void,
                    );
                }
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

enum BufferType {
    VERTEX,
    ELEMENT,
}

struct Buffer {
    handler: u32,
}

impl Buffer {
    pub fn new(buffer_type: BufferType, memory: &[u8]) -> Self {
        let mut handler = 0;
        let buffer_type = match buffer_type {
            BufferType::VERTEX => gl::ARRAY_BUFFER,
            BufferType::ELEMENT => gl::ELEMENT_ARRAY_BUFFER,
        };

        unsafe { gl::GenBuffers(1, &mut handler); }
        unsafe { gl::BindBuffer(buffer_type, handler); }
        unsafe { gl::BufferData(buffer_type, memory.len() as isize, memory.as_ptr() as *const c_void, gl::STATIC_DRAW); }

        Self { handler }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.handler); }
    }
}

enum VertexAttributeType {
    F32,
    I32,
    U32,
}

struct VertexAttribute {
    attribute_type: VertexAttributeType,
    size: usize,
    normalized: bool,
    stride: usize,
    width: usize,
}

impl VertexAttribute {
    fn new(attribute_type: VertexAttributeType, size: usize, normalized: bool, stride: usize, width: usize) -> Self {
        Self { attribute_type, size, normalized, stride, width }
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
        -0.5, -0.5, 0.,
        0.5, -0.5, 0.,
        -0.5, 0.5, 0.,
        0.5, 0.5, 0.,
    ];
    let indicies_buffer: Vec<u32> = vec![
        0, 1, 2,
        2, 1, 3,
    ];

    let vertex_buffer = vertex_buffer
        .into_iter()
        .flat_map(|v| v.to_ne_bytes())
        .collect::<Vec<_>>();
    let indicies_buffer = indicies_buffer
        .into_iter()
        .flat_map(|v| v.to_ne_bytes())
        .collect::<Vec<_>>();

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe { gl::BindVertexArray(vao); }

    let vbo = Buffer::new(BufferType::VERTEX, &vertex_buffer);
    let ebo = Buffer::new(BufferType::ELEMENT, &indicies_buffer);

    unsafe { gl::Enable(gl::CULL_FACE); }
    unsafe { gl::CullFace(gl::BACK); }

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

uniform vec3 color;

void main() {
    FragColor = vec4(color.xyz, 1.0f);
} 
    ";

    let fragment_shader = Shader::try_new(ShaderType::FRAGMENT, fragment_shader).unwrap();

    let mut shader_program = ShaderProgram::try_new(
        vertex_shader,
        fragment_shader,
        &[
            VertexAttribute::new(VertexAttributeType::F32, 3, false, 3 * size_of::<f32>(), 0),
        ]
    ).unwrap();
    shader_program.activate();

    let mut color: f32 = 0.;

    shader_program.set_3f32("color", 0.5, 0., 0.);

    let mut last_time = glfw.get_time();

    while !window.should_close() {
        let (width, height) = window.get_size();
        unsafe { gl::Viewport(0, 0, width, height); }

        let time_delta = glfw.get_time() - last_time;
        last_time = glfw.get_time();

        color = last_time.sin().abs() as f32;
        shader_program.set_3f32("color", color, 0., 0.);

        unsafe { gl::DrawElements(gl::TRIANGLES, indicies_buffer.len() as i32 / 4, gl::UNSIGNED_INT, null()); }

        window.swap_buffers();
        glfw.poll_events();

        // 60 fps max
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
