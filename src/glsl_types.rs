use gl;
use gl::types::*;

//////////////////////////////////////////
// Matrix structs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat2(pub [f32; 2 * 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat2x3(pub [f32; 2 * 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat2x4(pub [f32; 2 * 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat3x2(pub [f32; 3 * 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat3(pub [f32; 3 * 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat3x4(pub [f32; 3 * 4]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat4x2(pub [f32; 4 * 2]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat4x3(pub [f32; 4 * 3]);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat4(pub [f32; 4 * 4]);

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
    fn into_glsl_any(self) -> GLSLAny;
}

#[derive(Debug)]
pub enum GLSLAny {
    None,
    Float(f32),
    Vec2((f32, f32)),
    Vec3((f32, f32, f32)),
    Vec4((f32, f32, f32, f32)),
    Int(i32),
    Ivec2((i32, i32)),
    Ivec3((i32, i32, i32)),
    Ivec4((i32, i32, i32, i32)),
    Uint(u32),
    Uvec2((u32, u32)),
    Uvec3((u32, u32, u32)),
    Uvec4((u32, u32, u32, u32)),
    Bool(bool),
    Bvec2((bool, bool)),
    Bvec3((bool, bool, bool)),
    Bvec4((bool, bool, bool, bool)),
    Mat2(Mat2),
    Mat2x3(Mat2x3),
    Mat3x2(Mat3x2),
    Mat3(Mat3),
    Mat2x4(Mat2x4),
    Mat4x2(Mat4x2),
    Mat3x4(Mat3x4),
    Mat4x3(Mat4x3),
    Mat4(Mat4),
}

impl UniformType for f32 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Float(self)
    }
}

impl UniformType for (f32, f32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Vec2(self)
    }
}

impl UniformType for (f32, f32, f32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Vec3(self)
    }
}

impl UniformType for (f32, f32, f32, f32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Vec4(self)
    }
}

impl UniformType for i32 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Int(self)
    }
}

impl UniformType for (i32, i32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Ivec2(self)
    }
}

impl UniformType for (i32, i32, i32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Ivec3(self)
    }
}

impl UniformType for (i32, i32, i32, i32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Ivec4(self)
    }
}

impl UniformType for u32 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Uint(self)
    }
}

impl UniformType for (u32, u32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Uvec2(self)
    }
}

impl UniformType for (u32, u32, u32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Uvec3(self)
    }
}

impl UniformType for (u32, u32, u32, u32) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Uvec4(self)
    }
}

impl UniformType for bool {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Bool(self)
    }
}

impl UniformType for (bool, bool) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Bvec2(self)
    }
}

impl UniformType for (bool, bool, bool) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Bvec3(self)
    }
}

impl UniformType for (bool, bool, bool, bool) {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Bvec4(self)
    }
}

impl UniformType for Mat2 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat2(self)
    }
}

impl UniformType for Mat2x3 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat2x3(self)
    }
}

impl UniformType for Mat2x4 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat2x4(self)
    }
}

impl UniformType for Mat3x2 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat3x2(self)
    }
}

impl UniformType for Mat3 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat3(self)
    }
}

impl UniformType for Mat3x4 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat3x4(self)
    }
}

impl UniformType for Mat4x2 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat4x2(self)
    }
}

impl UniformType for Mat4x3 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat4x3(self)
    }
}

impl UniformType for Mat4 {
    fn into_glsl_any(self) -> GLSLAny {
        GLSLAny::Mat4(self)
    }
}
