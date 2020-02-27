/// NYAN NYAN NYAN!
/// ·.,¸,.·*¯`·.,¸,.·....╭━━━━╮
///`·.,¸,.·*¯`·.,¸,.·*¯. |::::::/\:__:/\
/// `·.,¸,.·*¯`·.,¸,.·* <|:::::(｡ ◕‿‿ ◕).
///  `·.,¸,.·*¯`·.,¸,.·* ╰O--O----O-O

pub mod buffers;
pub mod shaders;

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
