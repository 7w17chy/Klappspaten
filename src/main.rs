use glfw::{Action, Context, Key};
use gl::types::*;

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
    let positions: Vec<f32> = vec![
        -0.5, -0.5,
        0.5, -0.5,
        0.5,  0.5,
        -0.5,  0.5
    ];

    // create vertex buffer with vertex positons
    
    while !window.should_close() {
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

pub mod gl_helper_functions {
    use gl::types::*;
    /// Get gl string. since gl::GetString returns a pointer to the beginning of the actual
    /// string in bytes, you have to convert it to a Rust string before using it.
    pub(crate) fn get_gl_string(name: GLenum) -> String {
        let mut charbuff: Vec<u8> = Vec::new();

        unsafe {
            let mut ptr_string = gl::GetString(name);
            while *ptr_string as char != '\0' {
                charbuff.push(*ptr_string);
                ptr_string = ptr_string.wrapping_add(1); // increment pointer by 1 unit, not bytes
            }
            charbuff.push(b'\0');   // push '\0' to indicate the end of the string
        }
        
        match String::from_utf8(charbuff) {
            Ok(s) => s,
            Err(e) => panic!("Error reading opengl string: {}", e),
        }
    }
}

pub mod buffers {
    use gl::{self, types::GLenum};
    
    pub struct VertexBuffer {
        handle: *mut u32,
        is_bound: bool,
    }

    impl VertexBuffer {
        pub(crate) fn new(positions: &mut [f32]) -> VertexBuffer {
            let handle: *mut u32 = Vec::with_capacity(positions.len()).as_mut_ptr();
            unsafe {
                gl::GenBuffers(1, handle);
                gl::BindBuffer(gl::ARRAY_BUFFER, handle as u32);
                gl::BufferData(gl::ARRAY_BUFFER, (positions.len() * std::mem::size_of::<f32>()) as isize,
                               positions.as_mut_ptr() as *const core::ffi::c_void, gl::STATIC_DRAW);
                // std::mem::size_of<f32> * 2 => 2 floats per vertex
                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (std::mem::size_of::<f32>() * 2) as i32,
                                        0 as *const std::ffi::c_void);
                gl::EnableVertexAttribArray(0);
            }

            VertexBuffer {
                handle,
                is_bound: true,
            }
        }
        
        pub(crate) fn bind(&self, kind: GLenum) -> Result<(), &'static str> {
            // can't bind buffer that is already bound => throw an error
            if self.is_bound {
                return Err("Buffer already bound!");
            }

            unsafe { gl::BindBuffer(kind, self.handle as u32); }
            Ok(())
        }
    }
}

pub mod shaders {
    use gl::types::*;
    use std::ffi;
    
    pub struct Shader2D {
        handle: *mut u32,
        is_bound: bool,
    }

    // only for u8, i8 and char!
    pub struct ShaderSource<T> {
        src: T,
    }

    impl ShaderSource<i8> {
        pub(crate) fn to_cstr(&mut self) {
            self.src = match ffi::CStr::from_bytes_with_nul(&self.src) {
                Ok(cstr) => cstr,
                Err(e) => panic!("Failed to create shader source: {}\n Remember to end your shader source with \0!", e),
            };
        }
    }
    
    unsafe fn compile_shader(kind: GLenum, source: &ShaderSource<T>) -> u32 {
        let id: u32 = gl::CreateShader(kind);
        
        gl::ShaderSource(id, 1, &source.to_cstr().as_ptr(), 0 as *const _);
        gl::CompileShader(id);

        // TODO: Error handling (-> compilation errors)
        
        id
    }
    
    impl Shader2D {
        pub(crate) fn new(fragment_source: ShaderSource<T>, vertex_source: ShaderSource<T>) -> Shader2D {
            let handle: u32;
            
            unsafe {
                handle = gl::CreateProgram();
                let fs = compile_shader(gl::FRAGMENT_SHADER, fragment_source);
                let vs = compile_shader(gl::VERTEX_SHADER, vertex_source);

                // link vertex and fragment shader together into one shader program
                gl::AttachShader(handle, fs);
                gl::AttachShader(handle, vs);
                gl::LinkProgram(handle);
                gl::ValidateProgram(handle);

                // can be deleted now, they've been linked together before
                gl::DeleteShader(fs);
                gl::DeleteShader(vs);
            }
            
            Shader2D {
                handle,
                is_bound: true,
            }
        }

        pub(crate) fn bind(&self) {
            unsafe {
                gl::UseProgram(self.handle);
            }
        }
    }
}
