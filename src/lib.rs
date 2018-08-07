extern crate gl;

pub use self::buffer::*;
pub use self::draw_engine::*;
pub use self::draw_env::*;
pub use self::glsl_type::*;
pub use self::shader::*;
pub use self::texture::*;
pub use self::handles::*;

mod buffer;
mod draw_engine;
mod draw_env;
mod gl_buffer_resource;
mod gl_program_resource;
mod gl_shader_resource;
mod gl_texture_resource;
mod glsl_type;
mod handles;
mod shader;
mod texture;

// Ill write tests later
/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
