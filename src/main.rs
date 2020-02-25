use glfw::{Action, Context, Key, WindowHint};
use core::ffi::c_void;
use std::ffi::CString;
use ::ogl::{gl_helper_functions, buffers, shaders};
use std::process;

#[allow(unused_variables)]
fn main() {
    // initialising glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::OpenGlDebugContext(true));

    let (mut window, events) = glfw.create_window(300, 300, "Hello, world!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    // loading opengl function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // make OpenGL print error messages when an error occurs
    unsafe { gl::DebugMessageCallback(callbackfn, 0 as *mut c_void); }

    // print version of opengl to verify everything's setup correctly
    println!("OpenGL version: {}", gl_helper_functions::get_gl_string(gl::VERSION));

    // vertex positions
    let mut positions: Vec<f32> = vec![
        -0.5, -0.5,
         0.5, -0.5,
         0.5,  0.5,
        -0.5,  0.5
    ];

    // create vertex buffer with vertex positons. 2: 2 floats per vertex
    let vertbuf = buffers::VertexBuffer::new(&mut positions, 2);

    // get source for shaders
    let fragment_source = shaders::ShaderSource::from_file("fragment.glsl");
    let vertex_source = shaders::ShaderSource::from_file("vertex.glsl");
    
    // create shader
    let shader = shaders::Shader2D::new(fragment_source, vertex_source);
    
    // main loop
    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, (positions.len()/2) as i32);
        }
        // swap front and back buffers
        window.swap_buffers();

        // poll for any events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

/// Callback function for glDebugMessageCallback.
#[allow(unused_variables)]
extern "system" fn callbackfn(source: u32, gltype: u32, id: u32, severity: u32, len: i32,
                              message: *const i8, userparams: *mut c_void)
{
    let mut charbuff: Vec<u8> = Vec::new();
    let mut counter = 0; // counts up to the length of 'message'
    let mut ptr = message; // can't increment message itself, so increment a copy of it...

    unsafe {
        while counter < len {
            charbuff.push(*ptr as u8);
            ptr = ptr.wrapping_add(1); // increment pointer by 1 unit, not bytes
            counter += 1;
        }
    }

    let mess = match String::from_utf8(charbuff) {
        Ok(st) => st,
        Err(e) => panic!("GL error occurred (type: {:#x}), but was unable to convert the error message to a proper string! {}",
                         gltype, e),
    };
    
    eprintln!("\nGL error occurred!\nSource: {:#x}\nType: {:#x}\nId: {:#x}\nSeverity: {:#x}\n\nMessage: {}\n",
              source, gltype, id, severity, mess);

    // to prevent printing the message over and over again, abort the program
    process::abort();
}


fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
