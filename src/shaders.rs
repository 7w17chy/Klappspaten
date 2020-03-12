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
    pub fn bind(&mut self) {
        if self.is_bound {
            return;         // if it's already bound, do nothing
        }
        self.is_bound = true;
        unsafe { gl::UseProgram(self.handle); }
    }

    /// Get location of a uniform in the shader by its name. Shader must be bound. If it's not, the
    /// function will throw an error.
    pub fn get_uniform_location(&self, name: *const i8) -> Result<i32, &'static str> {
        if !self.is_bound { return Err("Shader must be bound!"); }

        unsafe { Ok(gl::GetUniformLocation(self.handle, name)) }
    }
}
