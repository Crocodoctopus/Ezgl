use super::buffer::*;
use super::gl_buffer_resource::*;
use super::glsl_types::*;
use super::program::*;
use super::texture::*;
use gl;
use gl::types::*;
use std::ffi::CStr;

pub struct InstantDraw<'a> {
    // necessary
    count: usize,
    draw_type: GLenum,
    program: Option<&'a Program>,
    ibo: Option<(&'a GLBufferResource, GLenum)>, // resource handle, type type

    // optional
    buffers: Vec<(&'a GLBufferResource, GLenum, GLint, GLenum, GLuint)>, // resource handle, buffer type, type count, type type, attribute #
    textures: Vec<(&'a Texture2D, GLint)>,
    uniforms: Vec<(GLSLAny, GLint)>,

    depth: Option<GLenum>,
    blend: Option<(GLenum, GLenum)>,
}

extern "system" fn callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _: *mut GLvoid,
) {
    unsafe {
        let rust_message = CStr::from_ptr(message).to_str().unwrap().to_owned();
        println!("A GL error has been thrown!");
        println!(
            "  source: {:?}, type: {:?}, id: {:?}, severity: {:?}",
            source, gltype, id, severity
        );
        println!("  Message: {:?}", rust_message);
    }
}

impl<'a> InstantDraw<'a> {
    // the dirtiest of hacks
    pub fn bind_vao() {
        static mut VAO: GLuint = 0;
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

    pub fn enable_debug() {
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(callback, 0 as _);
        }
    }

    pub fn new() -> Self {
        Self {
            count: 0,
            draw_type: 0,
            program: None,
            ibo: None,

            buffers: Vec::new(),
            textures: Vec::new(),
            uniforms: Vec::new(),

            depth: None,
            blend: None,
        }
    }

    pub fn with_count<T: ElementType + 'static>(
        &mut self,
        num: usize,
        draw_type: GLenum,
        program: &'a Program,
        ibo: &'a Buffer<T>,
    ) {
        self.count = num;
        self.draw_type = draw_type;
        self.program = Some(program);
        self.ibo = Some((&ibo.resource, T::get_type()));
    }

    pub fn add_buffer<T: BufferType + 'static>(&mut self, buffer: &'a Buffer<T>, loc: GLuint) {
        self.buffers.push((
            &buffer.resource,
            buffer.buffer_type,
            T::get_type().0,
            T::get_type().1,
            loc,
        ));
    }

    pub fn add_texture(&mut self, texture: &'a Texture2D, loc: GLint) {
        self.textures.push((texture, loc));
    }

    pub fn with_uniform(&mut self, t: GLSLAny, loc: GLint) {
        self.uniforms.push((t, loc));
    }

    pub fn enable_depth(&mut self, arg1: GLenum) {
        self.depth = Some(arg1);
    }

    pub fn enable_blend(&mut self, arg1: GLenum, arg2: GLenum) {
        self.blend = Some((arg1, arg2));
    }

    pub fn draw(mut self) {
        // extract the core resources
        let program = self.program.as_ref().expect("No program attached");
        let (ibo_resource, ibo_type_type) = self.ibo.expect("No ibo attached");

        unsafe {
            gl::UseProgram(program.resource.get_raw());
        }

        // check if the draw type and count is valid
        let mult = match self.draw_type {
            gl::POINTS => 1,
            gl::TRIANGLES => 3,
            _ => panic!("Invalid draw type"),
        };

        if self.count <= 0 {
            panic!("Invalid draw count: {}", self.count);
        }

        // bind the IBO
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo_resource.get_raw());
        }

        // attach the buffers
        // resource handle, buffer type, type count, type type, attribute #
        for (buffer_resource, buffer_type, buffer_type_count, buffer_type_type, loc) in self.buffers
        {
            unsafe {
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
        }

        // attach the textures
        let mut texture_target = 0;
        for (texture, loc) in self.textures {
            unsafe {
                gl::Uniform1i(loc, texture_target as _);
                gl::ActiveTexture(gl::TEXTURE0 + texture_target);
                gl::BindTexture(gl::TEXTURE_2D, texture.resource.get_raw());
            }

            texture_target += 1;
        }

        // depth
        unsafe {
            match self.depth {
                Some(arg1) => {
                    gl::Enable(gl::DEPTH_TEST);
                    gl::DepthFunc(arg1);
                }
                None => gl::Disable(gl::DEPTH_TEST),
            }
        }

        // blend
        unsafe {
            match self.blend {
                Some((arg1, arg2)) => {
                    gl::Enable(gl::BLEND);
                    gl::BlendFunc(arg1, arg2);
                }
                None => gl::Disable(gl::BLEND),
            }
        }

        // uniforms
        for (any, loc) in self.uniforms {
            unsafe {
                match any {
                    GLSLAny::Float(float) => gl::Uniform1f(loc, float),
                    GLSLAny::Vec2(vec2) => gl::Uniform2f(loc, vec2.0, vec2.1),
                    GLSLAny::Vec3(vec3) => gl::Uniform3f(loc, vec3.0, vec3.1, vec3.2),
                    GLSLAny::Vec4(vec4) => gl::Uniform4f(loc, vec4.0, vec4.1, vec4.2, vec4.3),
                    GLSLAny::Int(int) => gl::Uniform1i(loc, int),
                    GLSLAny::Ivec2(ivec2) => gl::Uniform2i(loc, ivec2.0, ivec2.1),
                    GLSLAny::Ivec3(ivec3) => gl::Uniform3i(loc, ivec3.0, ivec3.1, ivec3.2),
                    GLSLAny::Ivec4(ivec4) => gl::Uniform4i(loc, ivec4.0, ivec4.1, ivec4.2, ivec4.3),
                    GLSLAny::Uint(uint) => gl::Uniform1ui(loc, uint),
                    GLSLAny::Uvec2(uvec2) => gl::Uniform2ui(loc, uvec2.0, uvec2.1),
                    GLSLAny::Uvec3(uvec3) => gl::Uniform3ui(loc, uvec3.0, uvec3.1, uvec3.2),
                    GLSLAny::Uvec4(uvec4) => {
                        gl::Uniform4ui(loc, uvec4.0, uvec4.1, uvec4.2, uvec4.3)
                    }
                    GLSLAny::Bool(glbool) => gl::Uniform1ui(loc, glbool as _),
                    GLSLAny::Bvec2(bvec2) => gl::Uniform2ui(loc, bvec2.0 as _, bvec2.1 as _),
                    GLSLAny::Bvec3(bvec3) => {
                        gl::Uniform3ui(loc, bvec3.0 as _, bvec3.1 as _, bvec3.2 as _)
                    }
                    GLSLAny::Bvec4(bvec4) => {
                        gl::Uniform4ui(loc, bvec4.0 as _, bvec4.1 as _, bvec4.2 as _, bvec4.3 as _)
                    }
                    GLSLAny::Mat2(mat2) => {
                        gl::UniformMatrix2fv(loc, 1, gl::FALSE, &mat2 as *const _ as _)
                    }
                    GLSLAny::Mat3(mat3) => {
                        gl::UniformMatrix3fv(loc, 1, gl::FALSE, &mat3 as *const _ as _)
                    }
                    GLSLAny::Mat4(mat4) => {
                        gl::UniformMatrix4fv(loc, 1, gl::FALSE, &mat4 as *const _ as _)
                    }
                    GLSLAny::Mat2x3(mat2x3) => {
                        gl::UniformMatrix2x3fv(loc, 1, gl::FALSE, &mat2x3 as *const _ as _)
                    }
                    GLSLAny::Mat3x2(mat3x2) => {
                        gl::UniformMatrix3x2fv(loc, 1, gl::FALSE, &mat3x2 as *const _ as _)
                    }
                    GLSLAny::Mat2x4(mat2x4) => {
                        gl::UniformMatrix2x4fv(loc, 1, gl::FALSE, &mat2x4 as *const _ as _)
                    }
                    GLSLAny::Mat4x2(mat4x2) => {
                        gl::UniformMatrix4x2fv(loc, 1, gl::FALSE, &mat4x2 as *const _ as _)
                    }
                    GLSLAny::Mat3x4(mat3x4) => {
                        gl::UniformMatrix3x4fv(loc, 1, gl::FALSE, &mat3x4 as *const _ as _)
                    }
                    GLSLAny::Mat4x3(mat4x3) => {
                        gl::UniformMatrix4x3fv(loc, 1, gl::FALSE, &mat4x3 as *const _ as _)
                    }
                    GLSLAny::None => unreachable!(),
                }
            }
        }

        // draw
        unsafe {
            gl::DrawElements(
                self.draw_type,
                (self.count * mult) as _,
                ibo_type_type,
                (0 * mult) as _,
            );
        }
    }
}
