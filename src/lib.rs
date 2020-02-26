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

    /// Enum that specifies the type of buffer. Heavily used by BufferManager.
    pub enum BufferType {
       VertexBuffer,
       IndexBuffer,
    }

    /// Keeps track of how many (types of) buffers are currently around.
    pub struct BufferManager {
        /// Count of currently bound vertex buffers.
        vertex_buffer: u32,
        /// Count of currently bound index buffers.
        index_buffer: u32,
    }

    impl BufferManager {
        /// Creates a new BufferManager.
        pub fn new() -> Self {
            Self {
                vertex_buffer: 0,
                index_buffer: 0,
            }
        }

        /// This function should be called whenever a buffer is bound. As a parameter, it takes in
        /// the type of buffer in form of a BufferType. It returns the new size of currently bound
        /// buffers.
        pub fn increase(&mut self, btype: BufferType) -> u32 {
            match btype {
                // you have to specify the path of the variant or Rust will punish you with lots of
                // warnings...
                crate::buffers::BufferType::VertexBuffer => { 
                    self.vertex_buffer += 1;
                    return self.vertex_buffer;
                },
                crate::buffers::BufferType::IndexBuffer => {
                    self.vertex_buffer += 1;
                    return self.index_buffer;
                }
            };
        }

        /// This function should be called whenever a buffer is unbound. It decreases the number of
        /// currently bound buffers and returns it.
        pub fn decrease(&mut self, btype: BufferType) -> u32 {
            match btype {
                crate::buffers::BufferType::VertexBuffer => { 
                    self.vertex_buffer -= 1;
                    return self.vertex_buffer;
                },
                crate::buffers::BufferType::IndexBuffer => {
                    self.vertex_buffer -= 1;
                    return self.index_buffer;
                }
            };
        }
    }
    
    /// A representation of a vertex buffer.
    pub struct VertexBuffer {
        /// just the 'name' of the buffer, it's not a pointer! 
        pub handle: u32,        
        /// points to the handle (=> 'name')
        pub ptr: *mut u32,      
        /// indicates whether the buffer is bound or not
        pub is_bound: bool,     
    }

    impl VertexBuffer {
        /// Create vertex buffer, binding it in the process.
        pub fn new(positions: &mut [f32], floats_per_vertex: usize, bm: &mut BufferManager) -> VertexBuffer {
            // increase the count of currently bound vertex buffers and use the returned number as
            // id or name for the buffer
            let mut handle: u32 = bm.increase(BufferType::VertexBuffer); 
            let ptr: *mut u32 = &mut handle;
            unsafe {
                gl::GenBuffers(1, ptr);
                gl::BindBuffer(gl::ARRAY_BUFFER, handle);
                // provide information about the data stored in the buffer.
                gl::BufferData(gl::ARRAY_BUFFER, (positions.len() * std::mem::size_of::<f32>()) as isize,
                               positions.as_mut_ptr() as *const core::ffi::c_void, gl::STATIC_DRAW);
                // tell opengl how your data is layed out in memory.
                // std::mem::size_of::<f32> * 2 => 2 floats per vertex
                gl::VertexAttribPointer(0, floats_per_vertex as i32, gl::FLOAT, gl::FALSE, (std::mem::size_of::<f32>() * floats_per_vertex) as i32,
                                        0 as *const std::ffi::c_void);
                // 'bind' it on position 0
                gl::EnableVertexAttribArray(0);
            }

            // increase the count of currently bound buffers
            bm.increase(BufferType::VertexBuffer);

            VertexBuffer {
                handle,
                ptr,
                is_bound: true,
            }
        }
        
        /// Bind vertex buffer. If it's already bound, do nothing
        pub fn bind(&self, kind: GLenum, bm: &mut BufferManager) {
            // can't bind buffer that is already bound => do nothing
            if self.is_bound {
                return;
            }

            // increase the count of currently bound buffers...
            bm.increase(BufferType::VertexBuffer);

            unsafe { gl::BindBuffer(kind, self.handle); }
        }
    }

    /// Represents an index buffer.
    pub struct IndexBuffer {
        /// The 'id' or 'name' of the buffer, it's not a pointer but acts as one!
        pub name: u32,
        /// Pointer to the name. An awful lot of OpenGL functions require it...
        pub ptr: *mut u32,
        /// Indicator whether the index buffer is bound or not.
        pub is_bound: bool,
    }

    impl IndexBuffer {
        /// Create a new index buffer. indices.len() is a perfectly fine value for size.
        pub fn new(size: usize, indices: &mut Vec<u32>, bm: &mut BufferManager) -> Self {
            // increase the count of currently bound (will be bound in a few lines) index buffers +
            // return that number -- it will act as the id or name of the buffer.
            let mut name = bm.increase(BufferType::IndexBuffer);
            let ptr: *mut u32 = &mut name;
            unsafe {
                gl::GenBuffers(1, ptr);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, name);
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (size * std::mem::size_of::<u32>()) as isize,
                    indices.as_mut_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
            }
            
            IndexBuffer {
                name,
                ptr,
                is_bound: true,
            }
        }

        pub fn bind(&self, bm: &mut BufferManager) {
            if self.is_bound {
                return;
            }

            bm.increase(BufferType::IndexBuffer);
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.name); }
        }
    }
}

pub mod fileops {
    use std::ffi::CString;
    use std::fs;

    /// Read a file and return its contents in form of a std::ffi::CString. As such, it's easier
    /// to work with OpenGL function calls; you don't have to mess with converting it to a C-compatible
    /// string, it's already one. Plus you can easily convert it into a Rust string.
    pub fn read_file_into_cstring(filename: &str) -> CString {
        match fs::read(filename) {
            Ok(vec) => {
                // if a \0 character is found in the file you're reading, CString::new will return a NulError
                match CString::new(vec) {
                    Ok(c) => return c,
                    Err(e) => panic!("Don't put any \0 characters in your file, lad! {}", e),
                };
            }
            // you could match on the returned ErrorKind val, but for now, let's panic!
            Err(e) => panic!("Can't read file! {}", e),
        };
    }
}

pub mod shaders {
    use gl::types::*;
    use std::ffi::CString;
    
    /// A shader for 2d applications.
    pub struct Shader2D {
        /// The 'name' or 'id' of the shader
        pub handle: u32,        
        /// Will be 'true' when bound.
        pub is_bound: bool,     
    }

    /// Holds a CString that contains source code for a shader. Why a own struct just for that purpose?
    /// For uniformity and abstraction. All functions dealing with creating a shader will take in a ShaderSource.
    /// They don't have to deal with error checking or conversion of any kind, that's all done by the
    /// ShaderSource type, either by it's creation, or by providing methods.
    /// Also practical if you want to load all of your resources (obeject files, images, shader source, and so on)
    /// on a seperate thread or on startup for later usage.
    pub struct ShaderSource {
        /// String that contains source code for a shader.
        pub src: CString,       
    }

    impl ShaderSource {
        /// Create a ShaderSource instance from a file.
        pub fn from_file(filename: &str) -> Self {
            Self { src: super::fileops::read_file_into_cstring(filename), }
        }

        /// Create a ShaderSource instance from a Rust string.
        pub fn from_string(src: String) -> Self {
            let cstring = match CString::new(src) {
                Ok(cs) => cs,
                // CString doesn't like it when you put \0's in your files.
                Err(e) => panic!("Don't put an trailing '\0' in your source, lad! {}", e),
            };

            Self { src: cstring, }
        }

        /// Create a ShaderSource instance form a Vec<u8>
        pub fn from_byte_vec(src: Vec<u8>) -> Self {
            Self { src: unsafe { CString::from_vec_unchecked(src) }, }
        }
    }
    
    /// Helper function. Compile shader from given source and type.
    unsafe fn compile_shader(kind: GLenum, source: ShaderSource) -> u32 {
        let id: u32 = gl::CreateShader(kind);
        let pointer = source.src.as_ptr();

        gl::ShaderSource(id, 1, &pointer, 0 as *const _);
        gl::CompileShader(id);

        // TODO: Error handling (-> compilation errors)
        
        id
    }
    
    impl Shader2D {
        /// Create a shader program (vertex and fragment shader linked together) and bind it.
        pub fn new(fragment_source: ShaderSource, vertex_source: ShaderSource) -> Shader2D {
            let handle: u32; // the 'name' or 'id' of the shader
            
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
                is_bound: true, // shader was bound in the process of creating it 
            }
        }

        /// Bind shader. If it's already bound, do nothing.
        pub fn bind(&self) {
            if self.is_bound {
                return;         // if it's already bound, do nothing
            }
            unsafe {
                gl::UseProgram(self.handle);
            }
        }
    }
}
