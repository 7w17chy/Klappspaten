/// NYAN NYAN NYAN!
/// ·.,¸,.·*¯`·.,¸,.·....╭━━━━╮
///`·.,¸,.·*¯`·.,¸,.·*¯. |::::::/\:__:/\
/// `·.,¸,.·*¯`·.,¸,.·* <|:::::(｡ ◕‿‿ ◕).
///  `·.,¸,.·*¯`·.,¸,.·* ╰O--O----O-O

pub mod gl_helper_functions {
    use gl::types::GLenum;
    /// Get gl string. since gl::GetString returns a pointer to the beginning of the actual
    /// string in bytes, you have to convert it to a Rust string before using it.
    pub fn get_gl_string(name: GLenum) -> String {
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
        pub handle: u32,        // just the 'name' of the buffer, it's not a pointer! 
        pub ptr: *mut u32,      // points to the handle 
        pub is_bound: bool,
    }

    impl VertexBuffer {
        // TODO: currently throwing segfaults! (address boundary error)
        pub fn new(positions: &mut [f32]) -> VertexBuffer {
            let mut handle: u32 = 1; // if it's set to 0 and used with gl::BindBuffer, it will unbind all currently bound buffers!
            let ptr: *mut u32 = &mut handle;
            unsafe {
                gl::GenBuffers(1, ptr);
                gl::BindBuffer(gl::ARRAY_BUFFER, handle);
                gl::BufferData(gl::ARRAY_BUFFER, (positions.len() * std::mem::size_of::<f32>()) as isize,
                               positions.as_mut_ptr() as *const core::ffi::c_void, gl::STATIC_DRAW);
                // std::mem::size_of::<f32> * 2 => 2 floats per vertex
                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, (std::mem::size_of::<f32>() * 2) as i32,
                                        0 as *const std::ffi::c_void);
                gl::EnableVertexAttribArray(0);
            }

            VertexBuffer {
                handle,
                ptr,
                is_bound: true,
            }
        }
        
        pub fn bind(&self, kind: GLenum) {
            // can't bind buffer that is already bound => do nothing
            if self.is_bound {
                return;
            }

            unsafe { gl::BindBuffer(kind, self.handle); }
        }
    }
}

pub mod fileops {
    use std::ffi::CString;
    use std::fs;

    pub fn read_file_into_cstring(filename: &str) -> CString {
        match fs::read(filename) {
            Ok(vec) => return unsafe { CString::from_vec_unchecked(vec) },
            Err(e) => panic!("Can't read file! {}", e),
        };
    }
}

pub mod shaders {
    use gl::types::*;
    use std::ffi::CString;
    
    pub struct Shader2D {
        pub handle: u32,
        pub is_bound: bool,
    }

    pub struct ShaderSource {
        pub src: CString,
    }

    impl ShaderSource {
        pub fn from_file(filename: &str) -> Self {
            Self { src: super::fileops::read_file_into_cstring(filename), }
        }

        pub fn from_string(src: String) -> Self {
            let cstring = match CString::new(src) {
                Ok(cs) => cs,
                Err(e) => panic!("Don't put an trailing '\0' in you source, lad! {}", e),
            };

            Self { src: cstring, }
        }

        pub fn from_byte_vec(src: Vec<u8>) -> Self {
            Self { src: unsafe { CString::from_vec_unchecked(src) }, }
        }
    }
    
    unsafe fn compile_shader(kind: GLenum, source: ShaderSource) -> u32 {
        let id: u32 = gl::CreateShader(kind);
        let pointer = source.src.as_ptr();

        gl::ShaderSource(id, 1, &pointer, 0 as *const _);
        gl::CompileShader(id);

        // TODO: Error handling (-> compilation errors)
        
        id
    }
    
    impl Shader2D {
        pub fn new(fragment_source: ShaderSource, vertex_source: ShaderSource) -> Shader2D {
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

        pub fn bind(&self) {
            unsafe {
                gl::UseProgram(self.handle);
            }
        }
    }
}
