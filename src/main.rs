use glfw::{Action, Context, Key, WindowHint};
use core::ffi::c_void;
use ::ogl::{callbackfn, gl_helper_functions, buffers, shaders};

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
    unsafe { 
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(callbackfn, 0 as *mut c_void); 
    }

    // print version of opengl to verify everything's setup correctly
    println!("OpenGL version: {}", gl_helper_functions::get_gl_string(gl::VERSION));

    // vertex positions
    let mut positions: Vec<f32> = vec![
        -0.5, -0.5,
         0.5, -0.5,
         0.5,  0.5,
        -0.5,  0.5
    ];

    // indices for index buffer
    let mut indices: Vec<u32> = vec![
        0, 1, 2,
        2, 3, 0
    ];

    // create a buffer manager, that keeps track of how many buffers are currently around (->
    // bound, effectively)
    let mut buffman = buffers::BufferManager::new();

    // create vertex buffer with vertex positons. 2: 2 floats per vertex
    let vertbuf = buffers::VertexBuffer::new(&mut positions, 2, &mut buffman);

    // create index buffer (and bind it in the process)
    let indbuf = buffers::IndexBuffer::new(indices.len(), &mut indices, &mut buffman);

    // get source for shaders
    let fragment_source = shaders::ShaderSource::from_file("fragment.glsl");
    let vertex_source = shaders::ShaderSource::from_file("vertex.glsl");
    
    // create shader
    let shader = shaders::Shader2D::new(fragment_source, vertex_source);
    
    // main loop
    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            //gl::DrawArrays(gl::TRIANGLES, 0, (positions.len()/2) as i32);
            gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, 0 as *const _);
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

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
