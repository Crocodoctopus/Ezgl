pub extern crate gl;
extern crate png;

pub mod buffer;
pub mod draw;
pub mod gl_buffer_resource;
pub mod gl_program_resource;
pub mod gl_shader_resource;
pub mod gl_texture_resource;
pub mod glsl_types;
pub mod program;
pub mod shader;
pub mod texture;

pub use self::buffer::*;
pub use self::draw::*;
pub use self::glsl_types::*;
pub use self::program::*;
pub use self::shader::*;
pub use self::texture::*;

// the dirtiest of hacks
pub fn bind_vao() {
    static mut VAO: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::BindVertexArray(VAO);
    }
}

pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

extern "system" fn callback(
    source: gl::types::GLenum,
    gltype: gl::types::GLenum,
    id: gl::types::GLuint,
    severity: gl::types::GLenum,
    _length: gl::types::GLsizei,
    message: *const gl::types::GLchar,
    _: *mut gl::types::GLvoid,
) {
    unsafe {
        use std::ffi::CStr;
        let rust_message = CStr::from_ptr(message).to_str().unwrap().to_owned();
        println!("A GL error has been thrown!");
        println!(
            "  source: {:?}, type: {:?}, id: {:?}, severity: {:?}",
            source, gltype, id, severity
        );
        println!("  Message: {:?}", rust_message);
    }
}

pub fn enable_debug() {
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(callback, 0 as _);
    }
}
