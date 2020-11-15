use gl;
use gl::types::*;

//////////////////////////////////////////
// Matrix structs
pub type Mat2 = [[f32; 2]; 2];
pub type Mat2x3 = [[f32; 2]; 3];
pub type Mat2x4 = [[f32; 2]; 4];
pub type Mat3x2 = [[f32; 3]; 2];
pub type Mat3 = [[f32; 3]; 3];
pub type Mat3x4 = [[f32; 3]; 4];
pub type Mat4x2 = [[f32; 4]; 2];
pub type Mat4x3 = [[f32; 4]; 3];
pub type Mat4 = [[f32; 4]; 4];

//////////////////////////////////////////
// Buffer stuff
pub trait BufferType {
    fn get_type() -> (GLint, GLenum);
}

impl BufferType for f32 {
    fn get_type() -> (GLint, GLenum) {
        (1, gl::FLOAT)
    }
}

impl BufferType for (f32, f32) {
    fn get_type() -> (GLint, GLenum) {
        (2, gl::FLOAT)
    }
}

impl BufferType for (f32, f32, f32) {
    fn get_type() -> (GLint, GLenum) {
        (3, gl::FLOAT)
    }
}

impl BufferType for (f32, f32, f32, f32) {
    fn get_type() -> (GLint, GLenum) {
        (4, gl::FLOAT)
    }
}

impl BufferType for i32 {
    fn get_type() -> (GLint, GLenum) {
        (1, gl::INT)
    }
}

impl BufferType for (i32, i32) {
    fn get_type() -> (GLint, GLenum) {
        (2, gl::INT)
    }
}

impl BufferType for (i32, i32, i32) {
    fn get_type() -> (GLint, GLenum) {
        (3, gl::INT)
    }
}

impl BufferType for (i32, i32, i32, i32) {
    fn get_type() -> (GLint, GLenum) {
        (4, gl::INT)
    }
}

impl BufferType for u32 {
    fn get_type() -> (GLint, GLenum) {
        (1, gl::UNSIGNED_INT)
    }
}

impl BufferType for (u32, u32) {
    fn get_type() -> (GLint, GLenum) {
        (2, gl::UNSIGNED_INT)
    }
}

impl BufferType for (u32, u32, u32) {
    fn get_type() -> (GLint, GLenum) {
        (3, gl::UNSIGNED_INT)
    }
}

impl BufferType for (u32, u32, u32, u32) {
    fn get_type() -> (GLint, GLenum) {
        (4, gl::UNSIGNED_INT)
    }
}

impl BufferType for bool {
    fn get_type() -> (GLint, GLenum) {
        (1, gl::BOOL)
    }
}

impl BufferType for (bool, bool) {
    fn get_type() -> (GLint, GLenum) {
        (2, gl::BOOL)
    }
}

impl BufferType for (bool, bool, bool) {
    fn get_type() -> (GLint, GLenum) {
        (3, gl::BOOL)
    }
}

impl BufferType for (bool, bool, bool, bool) {
    fn get_type() -> (GLint, GLenum) {
        (4, gl::BOOL)
    }
}

impl BufferType for Mat2 {
    fn get_type() -> (GLint, GLenum) {
        (2 * 2, gl::FLOAT)
    }
}

impl BufferType for Mat2x3 {
    fn get_type() -> (GLint, GLenum) {
        (2 * 3, gl::FLOAT)
    }
}

impl BufferType for Mat2x4 {
    fn get_type() -> (GLint, GLenum) {
        (2 * 4, gl::FLOAT)
    }
}

impl BufferType for Mat3x2 {
    fn get_type() -> (GLint, GLenum) {
        (3 * 2, gl::FLOAT)
    }
}

impl BufferType for Mat3 {
    fn get_type() -> (GLint, GLenum) {
        (3 * 3, gl::FLOAT)
    }
}

impl BufferType for Mat3x4 {
    fn get_type() -> (GLint, GLenum) {
        (3 * 4, gl::FLOAT)
    }
}

impl BufferType for Mat4x2 {
    fn get_type() -> (GLint, GLenum) {
        (4 * 2, gl::FLOAT)
    }
}

impl BufferType for Mat4x3 {
    fn get_type() -> (GLint, GLenum) {
        (4 * 3, gl::FLOAT)
    }
}

impl BufferType for Mat4 {
    fn get_type() -> (GLint, GLenum) {
        (4 * 4, gl::FLOAT)
    }
}

//////////////////////////////////////////
// Element stuff
pub trait ElementType {
    fn get_type() -> GLenum;
}

impl ElementType for u8 {
    fn get_type() -> GLenum {
        gl::UNSIGNED_BYTE
    }
}

impl ElementType for u16 {
    fn get_type() -> GLenum {
        gl::UNSIGNED_SHORT
    }
}

impl ElementType for u32 {
    fn get_type() -> GLenum {
        gl::UNSIGNED_INT
    }
}

//////////////////////////////////////////
// Uniform stuff
pub trait UniformType {
    unsafe fn bind_uniform(&self, loc: GLint);
}

impl UniformType for f32 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform1f(loc, *self);
    }
}

impl UniformType for (f32, f32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform2f(loc, self.0, self.1);
    }
}

impl UniformType for (f32, f32, f32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform3f(loc, self.0, self.1, self.2);
    }
}

impl UniformType for (f32, f32, f32, f32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform4f(loc, self.0, self.1, self.2, self.3);
    }
}

impl UniformType for i32 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform1i(loc, *self)
    }
}

impl UniformType for (i32, i32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform2i(loc, self.0, self.1)
    }
}

impl UniformType for (i32, i32, i32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform3i(loc, self.0, self.1, self.2)
    }
}

impl UniformType for (i32, i32, i32, i32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform4i(loc, self.0, self.1, self.2, self.3)
    }
}

impl UniformType for u32 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform1ui(loc, *self)
    }
}

impl UniformType for (u32, u32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform2ui(loc, self.0, self.1)
    }
}

impl UniformType for (u32, u32, u32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform3ui(loc, self.0, self.1, self.2)
    }
}

impl UniformType for (u32, u32, u32, u32) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform4ui(loc, self.0, self.1, self.2, self.3)
    }
}

impl UniformType for bool {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform1ui(loc, *self as _)
    }
}

impl UniformType for (bool, bool) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform2ui(loc, self.0 as _, self.1 as _)
    }
}

impl UniformType for (bool, bool, bool) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform3ui(loc, self.0 as _, self.1 as _, self.2 as _)
    }
}

impl UniformType for (bool, bool, bool, bool) {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::Uniform4ui(loc, self.0 as _, self.1 as _, self.2 as _, self.3 as _)
    }
}

impl UniformType for Mat2 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix2fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat2x3 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix2x3fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat2x4 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix2x4fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat3x2 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix3x2fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat3 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix3fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat3x4 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix3x4fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat4x2 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix4x2fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat4x3 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix4x3fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}

impl UniformType for Mat4 {
    unsafe fn bind_uniform(&self, loc: GLint) {
        gl::UniformMatrix4fv(loc, 1, gl::FALSE, self as *const _ as _)
    }
}
