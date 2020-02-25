use glfw::{Action, Context, Key};
use ::ogl::{gl_helper_functions, buffers, shaders};

#[allow(unused_variables)]
fn main() {
    // initialising glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello, world!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    
    // loading opengl function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    
    // print version of opengl to verify everything's setup correctly
    println!("OpenGL version: {}", gl_helper_functions::get_gl_string(gl::VERSION));

    // vertex positions
    #[allow(unused_variables)]
    let mut positions: Vec<f32> = vec![
        -0.5, -0.5,
         0.5, -0.5,
         0.5,  0.5,
        -0.5,  0.5
    ];

    // create vertex buffer with vertex positons
    let vertbuf = buffers::VertexBuffer::new(&mut positions);

    // get source for shaders
    let fragment_source = shaders::ShaderSource::from_file("fragment.glsl"); // currently throwing segfaults
    let vertex_source = shaders::ShaderSource::from_file("vertex.glsl");
    
//    let vertex_source = shaders::ShaderSource::from_string(String::from(
//        "
//#version 330 core\n
//layout(location = 0) in vec4 postion;\n
//void main()\n
//{\n
//   gl_Position = postion;\n
//}\n;
//"
//    ));
//
//    let fragment_source = shaders::ShaderSource::from_string(String::from(
//        "
//  #version 330 core\n
// layout(location = 0) out vec4 color;\n
// void main()\n
// {\n
//    color = vec4(1.0, 0.0, 0.0, 1.0);\n
// }\n
//"
//    ));

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

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
