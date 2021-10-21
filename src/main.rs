extern crate glfw;
use std::{borrow::BorrowMut, ffi::{CStr, CString, c_void}, mem::size_of, ptr::{null, null_mut}, slice};

use gl::types::{GLchar, GLint};

use self::glfw::Context;

extern crate gl;

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
    let vertex_shader = CString::new(vertex_shader.as_bytes()).expect("shader is not UTF-8 string");
    let vertex_shader_s;
    unsafe { vertex_shader_s = gl::CreateShader(gl::VERTEX_SHADER); }
    unsafe { gl::ShaderSource(vertex_shader_s, 1, &vertex_shader.as_ptr(), null()); }
    unsafe { gl::CompileShader(vertex_shader_s); }


    let mut success = gl::TRUE as GLint;
    unsafe { gl::GetShaderiv(vertex_shader_s, gl::COMPILE_STATUS, &mut success); }
    if success != gl::TRUE as GLint {
        const LEN: usize = 512;
        let mut log: Vec<u8> = Vec::with_capacity(LEN);
        unsafe { log.set_len(LEN); }
        unsafe { gl::GetShaderInfoLog(vertex_shader_s, LEN as i32, null_mut(), log.as_mut_ptr() as *mut GLchar); }

        let log = match std::str::from_utf8(&log) {
            Ok(s) => s,
            Err(e) => std::str::from_utf8(&log[..e.valid_up_to()]).unwrap()
        };

        panic!("Shader compilation failed:\n{}", log);
    }

    let fragment_shader = "
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
    ";

    let fragment_shader = CString::new(fragment_shader.as_bytes()).expect("shader is not UTF-8 string");
    let fragment_shader_s;
    unsafe { fragment_shader_s = gl::CreateShader(gl::FRAGMENT_SHADER); }
    unsafe { gl::ShaderSource(fragment_shader_s, 1, &fragment_shader.as_ptr(), null()); }
    unsafe { gl::CompileShader(fragment_shader_s); }

    let mut success = gl::TRUE as GLint;
    unsafe { gl::GetShaderiv(fragment_shader_s, gl::COMPILE_STATUS, &mut success); }
    if success != gl::TRUE as GLint {
        const LEN: usize = 512;
        let mut log: Vec<u8> = Vec::with_capacity(LEN);
        unsafe { log.set_len(LEN); }
        unsafe { gl::GetShaderInfoLog(fragment_shader_s, LEN as i32, null_mut(), log.as_mut_ptr() as *mut GLchar); }

        let log = match std::str::from_utf8(&log) {
            Ok(s) => s,
            Err(e) => std::str::from_utf8(&log[..e.valid_up_to()]).unwrap()
        };

        panic!("Shader compilation failed:\n{}", log);
    }


    let shader_program;
    unsafe { shader_program = gl::CreateProgram(); }

    unsafe { gl::AttachShader(shader_program, vertex_shader_s); }
    unsafe { gl::AttachShader(shader_program, fragment_shader_s); }
    unsafe { gl::LinkProgram(shader_program); }

    let mut success = gl::TRUE as GLint;
    unsafe { gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success); }
    if success != gl::TRUE as GLint {
        const LEN: usize = 512;
        let mut log: Vec<u8> = Vec::with_capacity(LEN);
        unsafe { log.set_len(LEN); }
        unsafe { gl::GetProgramInfoLog(shader_program, LEN as i32, null_mut(), log.as_mut_ptr() as *mut GLchar); }

        let log = match std::str::from_utf8(&log) {
            Ok(s) => s,
            Err(e) => std::str::from_utf8(&log[..e.valid_up_to()]).unwrap()
        };

        panic!("Shader linkage failed:\n{}", log);
    }

    unsafe { gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>() as i32, null()); }
    unsafe { gl::EnableVertexAttribArray(0); }

    unsafe { gl::UseProgram(shader_program); }
    unsafe { gl::DeleteShader(vertex_shader_s); }
    unsafe { gl::DeleteShader(fragment_shader_s); }


    while !window.should_close() {
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3); }

        window.swap_buffers();
        glfw.poll_events();
    }
}
