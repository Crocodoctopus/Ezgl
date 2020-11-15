use super::buffer::*;
use super::gl_buffer_resource::*;
use super::glsl_types::*;
use super::program::*;
use super::texture::*;
use gl;
use gl::types::*;

pub struct Draw<'a> {
    // necessary
    count: u32,
    draw_type: GLenum,
    program: &'a Program,
    ibo: (&'a GLBufferResource, GLenum), // resource handle, type type

    // optional
    buffers: Vec<(&'a GLBufferResource, GLenum, GLint, GLenum, GLuint)>, // resource handle, buffer type, type count, type type, attribute #
    textures: Vec<(&'a Texture2D, GLint)>,
    uniforms: Vec<(&'a UniformType, GLint)>,

    depth: Option<GLenum>,
    blend: Option<(GLenum, GLenum)>,
}

impl<'a> Draw<'a> {
    pub fn start_tri_draw<T: ElementType + 'static>(
        count: u32,
        program: &'a Program,
        ibo: &'a Buffer<T>,
    ) -> Self {
        Self {
            count: count * 3,
            draw_type: gl::TRIANGLES,
            program,
            ibo: (&ibo.resource, T::get_type()),

            buffers: Vec::new(),
            textures: Vec::new(),
            uniforms: Vec::new(),

            depth: None,
            blend: None,
        }
    }

    pub fn with_buffer<T: BufferType + 'static>(
        mut self,
        buffer: &'a Buffer<T>,
        loc: GLuint,
    ) -> Self {
        self.buffers.push((
            &buffer.resource,
            buffer.buffer_type,
            T::get_type().0,
            T::get_type().1,
            loc,
        ));
        self
    }

    pub fn with_texture(mut self, texture: &'a Texture2D, loc: GLint) -> Self {
        self.textures.push((texture, loc));
        self
    }

    pub fn with_uniform(mut self, t: &'a UniformType, loc: GLint) -> Self {
        self.uniforms.push((t, loc));
        self
    }

    pub fn enable_depth(mut self, arg1: GLenum) -> Self {
        self.depth = Some(arg1);
        self
    }

    pub fn enable_blend(mut self, arg1: GLenum, arg2: GLenum) -> Self {
        self.blend = Some((arg1, arg2));
        self
    }

    pub fn draw(self) {
        // if draw count is 0, do nothing
        if self.count == 0 {
            return;
        }

        // unsafe time
        unsafe {
            // bind program
            gl::UseProgram(self.program.resource.get_raw());

            // bind element buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo.0.get_raw());

            // bind other buffers
            for (buffer_resource, buffer_type, buffer_type_count, buffer_type_type, loc) in
                self.buffers
            {
                gl::BindBuffer(buffer_type, buffer_resource.get_raw());
                gl::EnableVertexAttribArray(loc);
                match buffer_type_type {
                    gl::BYTE
                    | gl::UNSIGNED_BYTE
                    | gl::SHORT
                    | gl::UNSIGNED_SHORT
                    | gl::INT
                    | gl::UNSIGNED_INT => {
                        gl::VertexAttribIPointer(
                            loc,
                            buffer_type_count,
                            buffer_type_type,
                            0,
                            0u32 as _,
                        );
                    }
                    _ => {
                        gl::VertexAttribPointer(
                            loc,
                            buffer_type_count,
                            buffer_type_type,
                            gl::FALSE,
                            0,
                            0u32 as _,
                        );
                    }
                }
            }

            // uniforms
            for (uniform, loc) in self.uniforms {
                uniform.bind_uniform(loc);
            }

            // attach textures
            let mut texture_target = 0;
            for (texture, loc) in self.textures {
                gl::Uniform1i(loc, texture_target as _);
                gl::ActiveTexture(gl::TEXTURE0 + texture_target);
                gl::BindTexture(gl::TEXTURE_2D, texture.resource.get_raw());

                texture_target += 1;
            }

            // depth
            match self.depth {
                Some(arg1) => {
                    gl::Enable(gl::DEPTH_TEST);
                    gl::DepthFunc(arg1);
                }
                None => gl::Disable(gl::DEPTH_TEST),
            }

            // blend
            match self.blend {
                Some((arg1, arg2)) => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(arg1, arg2);
                }
                None => gl::Disable(gl::BLEND),
            }

            // draw
            gl::DrawElements(self.draw_type, self.count as _, self.ibo.1, 0 as _);
        }
    }
}
