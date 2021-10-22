extern crate glfw;
use std::{io::BufReader, fs::File, time::Duration};

use cardless::simple2d_renderer::BatchRenderer;
use glm::vec2;

use crate::cardless::texture::Texture;

use self::glfw::Context;


mod cardless;
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

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao); }
    unsafe { gl::BindVertexArray(vao); }



    unsafe { gl::Enable(gl::CULL_FACE); }
    unsafe { gl::CullFace(gl::BACK); }

    let vertex_shader = "
#version 330 core
layout (location = 0) in vec2 vert_pos;
layout (location = 1) in vec2 vert_uv;
layout (location = 2) in int vert_texture;

out vec2 frag_uv;
flat out int frag_texture;

void main() {
    frag_uv = vert_uv;
    frag_texture = vert_texture;
    gl_Position = vec4(vert_pos.xy, 0.0, 1.0);
}
        ";

    let fragment_shader = "
#version 330 core
in vec2 frag_uv;
flat in int frag_texture;

out vec4 finale_color;

uniform sampler2D u_texture[16];

void main() {
    vec4 v_color = vec4(0.4, 0.6, 0.2, 1);
    switch(frag_texture) {
        case 0: v_color = texture(u_texture[0], frag_uv); break;
        case 1: v_color = texture(u_texture[1], frag_uv); break;
        case 2: v_color = texture(u_texture[2], frag_uv); break;
        case 3: v_color = texture(u_texture[3], frag_uv); break;
        case 4: v_color = texture(u_texture[4], frag_uv); break;
        case 5: v_color = texture(u_texture[5], frag_uv); break;
        case 6: v_color = texture(u_texture[6], frag_uv); break;
        case 7: v_color = texture(u_texture[7], frag_uv); break;
        case 8: v_color = texture(u_texture[8], frag_uv); break;
        case 9: v_color = texture(u_texture[9], frag_uv); break;
        case 10: v_color = texture(u_texture[10], frag_uv); break;
        case 11: v_color = texture(u_texture[11], frag_uv); break;
        case 12: v_color = texture(u_texture[12], frag_uv); break;
        case 13: v_color = texture(u_texture[13], frag_uv); break;
        case 14: v_color = texture(u_texture[14], frag_uv); break;
        case 15: v_color = texture(u_texture[15], frag_uv); break;
    }

    finale_color = v_color;
    // finale_color = vec4(vec3(frag_uv, .0) / (frag_texture + 1), 1);
    // finale_color = vec4(vec3((frag_texture + 1) / 3.0), 1.0);
} 
    ";

    let mut br = BatchRenderer::new(fragment_shader, vertex_shader);
    let image_a = File::open("./sample_texture_0.png").unwrap();
    let image_a = Texture::try_load(BufReader::new(image_a)).unwrap();
    let image_b = File::open("./sample_texture_1.png").unwrap();
    let image_b = Texture::try_load(BufReader::new(image_b)).unwrap();
    let image_c = File::open("./sample_texture_2.png").unwrap();
    let image_c = Texture::try_load(BufReader::new(image_c)).unwrap();


    unsafe { gl::Enable(gl::BLEND); }
    unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); }

    // 0 stands for TEXTURE0, so it applies image_a
    // 1 stands for TEXTURE1, therefore it applies image_b
    // shader_program.set_1i("u_texture", 0 as i32);

    let mut time_last_update = glfw.get_time();


    br.bind();
    while !window.should_close() {
        let time_now = glfw.get_time();
        let time_delta = glfw.get_time() - time_last_update;
        time_last_update = time_now;

        let (width, height) = window.get_size();
        unsafe { gl::Viewport(0, 0, width, height); }


        unsafe { gl::ClearColor(0.6, 0.2, 0.6, 1.); }
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

        br.push_square_texture(vec2(-0.4, -0.4), vec2(0.2, 0.2), &image_a);
        br.push_square_texture(vec2(0.4, time_now.sin() as f32), vec2(0.2, 0.2), &image_b);
        br.push_square_texture(vec2(-0.4, 0.4), vec2(0.2, 0.2), &image_a);
        br.push_square_texture(vec2(0.4, 0.4), vec2(0.2, 0.2), &image_b);
        br.push_square_texture(vec2(0.0, 0.0), vec2(0.2, 0.2), &image_c);

        br.flush();

        window.swap_buffers();
        glfw.poll_events();

        // 60 fps max
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
}
