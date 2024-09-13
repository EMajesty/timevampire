extern crate glfw;
use glfw::Context;

use glfw::ffi::glfwGetTime;

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
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw.create_window(SCR_WT, SCR_HT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);

    // Load gl functions
    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::ClearColor(0.3, 0.4, 0.6, 1.0);
    }

    let vertex_shader_src: &str = load_str!("vertex.glsl");
    let fragment_shader_src: &str = load_str!("fragment.glsl");

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

    while !window.should_close() {
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event)
        }

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.6, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);



        // update the uniform color
        let timeValue: f32 = glfwGetTime() as f32;
        let greenValue = timeValue.sin() / 2.0 + 0.5;
        
        unsafe {
            let c_string = CString::new("vertexColor").unwrap();
            let vertexColor: *const i8 = c_string.as_ptr();
       
            let vertexColorLocation = GetUniformLocation(shader_program, vertexColor);
            Uniform4f(vertexColorLocation, 0.0, greenValue, 0.0, 1.0);
        }

        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        gl::BindVertexArray(0);
        }

        window.swap_buffers();
    }
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::WindowEvent as Event;
    use glfw::Key;
    use glfw::Action;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        },
        _ => {},
    }
}
