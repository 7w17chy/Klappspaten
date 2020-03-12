/// NYAN NYAN NYAN!
/// ·.,¸,.·*¯`·.,¸,.·....╭━━━━╮
///`·.,¸,.·*¯`·.,¸,.·*¯. |::::::/\:__:/\
/// `·.,¸,.·*¯`·.,¸,.·* <|:::::(｡ ◕‿‿ ◕).
///  `·.,¸,.·*¯`·.,¸,.·* ╰O--O----O-O

pub mod buffers;
pub mod shaders;

use core::ffi::c_void;
use std::process;
use gl;

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

/// Callback function for glDebugMessageCallback.
#[allow(unused_variables)]
pub extern "system" fn callbackfn(source: u32, gltype: u32, id: u32, severity: u32, len: i32,
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

    if severity == gl::DEBUG_SEVERITY_MEDIUM { eprintln!("Keep going. Severity level: DEBUG_SEVERITY_MEDIUM"); }
    else if severity == gl::DEBUG_SEVERITY_LOW { eprintln!("Keep going. Severity level: DEBUG_SEVERITY_LOW"); }
    else if severity == gl::DEBUG_SEVERITY_NOTIFICATION { eprintln!("Keep going. Severity level: DEBUG_SEVERITY_NOTIFICATION"); }

    if source == gl::DEBUG_SOURCE_API { eprintln!("Source: DEBUG_SOURCE_API"); }
    else if source == gl::DEBUG_SOURCE_WINDOW_SYSTEM { eprintln!("Source: DEBUG_SOURCE_WINDOW_SYSTEM"); }
    else if source == gl::DEBUG_SOURCE_SHADER_COMPILER { eprintln!("Source: DEBUG_SOURCE_SHADER_COMPILER"); }
    else if source == gl::DEBUG_SOURCE_THIRD_PARTY { eprintln!("Source: DEBUG_SOURCE_THIRD_PARTY"); }
    else if source == gl::DEBUG_SOURCE_APPLICATION { eprintln!("Source: DEBUG_SOURCE_APPLICATION"); }
    else if source == gl::DEBUG_SOURCE_OTHER { eprintln!("Source: DEBUG_SOURCE_OTHER"); }

    if gltype == gl::DEBUG_TYPE_ERROR { eprintln!("Type: DEBUG_TYPE_ERROR"); }
    else if gltype == gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR { eprintln!("Type: DEBUG_TYPE_DEPRECATED_BEHAVIOR"); }
    else if gltype == gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR { eprintln!("Type: DEBUG_TYPE_UNDEFINED_BEHAVIOR"); }
    else if gltype == gl::DEBUG_TYPE_PORTABILITY { eprintln!("Type: DEBUG_TYPE_PORTABILITY"); }
    else if gltype == gl::DEBUG_TYPE_PERFORMANCE { eprintln!("Type: DEBUG_TYPE_PERFORMANCE"); }
    else if gltype == gl::DEBUG_TYPE_MARKER { eprintln!("Type: DEBUG_TYPE_MARKER"); }
    else if gltype == gl::DEBUG_TYPE_PUSH_GROUP { eprintln!("Type: DEBUG_TYPE_PUSH_GROUP"); }
    else if gltype == gl::DEBUG_TYPE_POP_GROUP { eprintln!("Type: DEBUG_TYPE_POP_GROUP"); }
    else if gltype == gl::DEBUG_TYPE_OTHER { eprintln!("Type: DEBUG_TYPE_OTHER"); }

    let mess = match String::from_utf8(charbuff) {
        Ok(st) => st,
        Err(e) => panic!("GL error occurred (type: {:#x}), but was unable to convert the error message to a proper string! {}",
                         gltype, e),
    };

    eprintln!("Id: {:#x}\nMessage: {}", id, mess);
    if severity == gl::DEBUG_SEVERITY_HIGH {
        eprintln!("aborting due to error (gl::DEBUG_SEVERITY_HIGH)");
        process::exit(1);
    }
}
