// extern crate glfw;
// use glfw::Context;
//
// use glfw::ffi::glfwGetTime;
//
extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::SDL_GetTicks;
use std::time::Duration;

extern crate gl;
// use gl::types::*;

use gl::GetUniformLocation;
use gl::Uniform4f;

use std::ffi::CString;

use load_file::load_str;

const SCR_WT: u32 = 800;
const SCR_HT: u32 = 600;
const TITLE: &str = "Hell World";

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(TITLE, SCR_WT, SCR_HT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let (screen_width, screen_height) = canvas.window().size();

    // Load gl functions
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width as i32, screen_height as i32);
    }

    let vertex_shader_src: &str = load_str!(".vert");
    let fragment_shader_src: &str = load_str!(".frag");

    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
    unsafe {
        gl::ShaderSource(vertex_shader, 1, &vertex_shader_src.as_bytes().as_ptr().cast(), &vertex_shader_src.len().try_into().unwrap());
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log_len = 0_i32;
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex shader compile error: {}", String::from_utf8_lossy(&v));
        }
    }

    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        gl::ShaderSource(fragment_shader, 1, &fragment_shader_src.as_bytes().as_ptr().cast(), &fragment_shader_src.len().try_into().unwrap());
        gl::CompileShader(fragment_shader);

        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment shader compile error: {}", String::from_utf8_lossy(&v));
        }
    }

    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program link error: {}", String::from_utf8_lossy(&v));
        }

        gl::DetachShader(shader_program, vertex_shader);
        gl::DetachShader(shader_program, fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    let vertices = [
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0f32,
    ];

    let mut vao = 0;
    unsafe { gl::GenVertexArrays(1, &mut vao) };

    let mut vbo = 0;
    unsafe { gl::GenBuffers(1, &mut vbo) };

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(&vertices)as isize,
        vertices.as_ptr().cast(), gl::STATIC_DRAW);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, 0 as *const _);
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 -i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Game loop
        unsafe {
            gl::ClearColor(0.3, 0.4, 0.6, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);



            // update the uniform color
            let time_value: f32 = SDL_GetTicks() as f32;
            let red_value = time_value.sin() / 2.0 + 0.5;
            let green_value = -time_value.sin() / 2.0 + 0.5;
            let blue_value = time_value * time_value.sin() / 2.0 + 0.5;
            
            let c_string = CString::new("vertexColor").unwrap();
            let vertex_color: *const i8 = c_string.as_ptr();
        
            let vertex_color_location = GetUniformLocation(shader_program, vertex_color);
            Uniform4f(vertex_color_location, red_value, green_value, blue_value, 1.0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindVertexArray(0);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
