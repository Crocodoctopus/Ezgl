use gl;
use gl::types::*;

pub enum GLSLAny {
	None,
	Float(Float),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Int(Int),
	Ivec2(Ivec2),
	Ivec3(Ivec3),
	Ivec4(Ivec4),
	Uint(Uint),
	Uvec2(Uvec2),
	Uvec3(Uvec3),
	Uvec4(Uvec4),
	Bool(Bool),
	Bvec2(Bvec2),
	Bvec3(Bvec3),
	Bvec4(Bvec4),
	Mat2(Mat2),
	Mat3(Mat3),
	Mat4(Mat4),
	Mat2x3(Mat2x3),
	Mat3x2(Mat3x2),
	Mat2x4(Mat2x4),
	Mat4x2(Mat4x2),
	Mat3x4(Mat3x4),
	Mat4x3(Mat4x3),
}

// buffer types
pub trait GLSLType {
	fn get_type() -> (GLint, GLenum);
	fn into_glsl_any(self) -> GLSLAny;
}

// bool
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bool(pub GLboolean);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bvec2(pub [GLboolean; 2]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bvec3(pub [GLboolean; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bvec4(pub [GLboolean; 4]);

// int
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Int(pub GLint);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ivec2(pub [GLint; 2]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ivec3(pub [GLint; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ivec4(pub [GLint; 4]);

// uint
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uint(pub GLuint);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uvec2(pub [GLuint; 2]);
pub type Uvec3 = [GLuint; 3];
//pub struct Uvec3(pub [GLuint; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Uvec4(pub [GLuint; 4]);

// float
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Float(pub GLfloat);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec2(pub [GLfloat; 2]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3(pub [GLfloat; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec4(pub [GLfloat; 4]);

// double
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Double(pub GLdouble);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dvec2(pub [GLdouble; 2]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dvec3(pub [GLdouble; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dvec4(pub [GLdouble; 4]);

// float mat
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat2(pub [GLfloat; 4]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat2x3(pub [GLfloat; 6]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat2x4(pub [GLfloat; 8]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat3x2(pub [GLfloat; 6]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat3(pub [GLfloat; 9]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat3x4(pub [GLfloat; 12]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat4x2(pub [GLfloat; 8]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat4x3(pub [GLfloat; 12]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mat4(pub [GLfloat; 16]);

// double mat
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat2(pub [GLdouble; 4]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat2x3(pub [GLdouble; 6]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat2x4(pub [GLdouble; 8]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat3x2(pub [GLdouble; 6]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat3(pub [GLdouble; 9]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat3x4(pub [GLdouble; 12]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat4x2(pub [GLdouble; 8]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat4x3(pub [GLdouble; 12]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dmat4(pub [GLdouble; 16]);

// tris and shorts (element use only)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ByteTri(pub [GLubyte; 3]);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShortTri(pub [GLushort; 3]);

// impls
impl GLSLType for Bool {
	fn get_type() -> (GLint, GLenum) {
		(1, gl::BOOL)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Bool(self)
	}
}

impl GLSLType for Int {
	fn get_type() -> (GLint, GLenum) {
		(1, gl::INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Int(self)
	}
}

impl GLSLType for Uint {
	fn get_type() -> (GLint, GLenum) {
		(1, gl::UNSIGNED_INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Uint(self)
	}
}

impl GLSLType for Float {
	fn get_type() -> (GLint, GLenum) {
		(1, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Float(self)
	}
}

impl GLSLType for Double {
	fn get_type() -> (GLint, GLenum) {
		(1, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Bvec2 {
	fn get_type() -> (GLint, GLenum) {
		(2, gl::BOOL)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Bvec2(self)
	}
}

impl GLSLType for Bvec3 {
	fn get_type() -> (GLint, GLenum) {
		(3, gl::BOOL)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Bvec3(self)
	}
}

impl GLSLType for Bvec4 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::BOOL)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Bvec4(self)
	}
}

impl GLSLType for Ivec2 {
	fn get_type() -> (GLint, GLenum) {
		(2, gl::INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Ivec2(self)
	}
}

impl GLSLType for Ivec3 {
	fn get_type() -> (GLint, GLenum) {
		(3, gl::INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Ivec3(self)
	}
}

impl GLSLType for Ivec4 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Ivec4(self)
	}
}

impl GLSLType for Uvec2 {
	fn get_type() -> (GLint, GLenum) {
		(2, gl::UNSIGNED_INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Uvec2(self)
	}
}

impl GLSLType for Uvec3 {
	fn get_type() -> (GLint, GLenum) {
		(3, gl::UNSIGNED_INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Uvec3(self)
	}
}

impl GLSLType for Uvec4 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::UNSIGNED_INT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Uvec4(self)
	}
}

impl GLSLType for Vec2 {
	fn get_type() -> (GLint, GLenum) {
		(2, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Vec2(self)
	}
}

impl GLSLType for Vec3 {
	fn get_type() -> (GLint, GLenum) {
		(3, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Vec3(self)
	}
}

impl GLSLType for Vec4 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Vec4(self)
	}
}

impl GLSLType for Dvec2 {
	fn get_type() -> (GLint, GLenum) {
		(2, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dvec3 {
	fn get_type() -> (GLint, GLenum) {
		(3, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dvec4 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Mat2 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat2(self)
	}
}

impl GLSLType for Mat2x3 {
	fn get_type() -> (GLint, GLenum) {
		(6, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat2x3(self)
	}
}

impl GLSLType for Mat2x4 {
	fn get_type() -> (GLint, GLenum) {
		(8, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat2x4(self)
	}
}

impl GLSLType for Mat3x2 {
	fn get_type() -> (GLint, GLenum) {
		(6, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat3x2(self)
	}
}

impl GLSLType for Mat3 {
	fn get_type() -> (GLint, GLenum) {
		(9, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat3(self)
	}
}

impl GLSLType for Mat3x4 {
	fn get_type() -> (GLint, GLenum) {
		(12, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat3x4(self)
	}
}

impl GLSLType for Mat4x2 {
	fn get_type() -> (GLint, GLenum) {
		(8, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat4x2(self)
	}
}

impl GLSLType for Mat4x3 {
	fn get_type() -> (GLint, GLenum) {
		(12, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat4x3(self)
	}
}

impl GLSLType for Mat4 {
	fn get_type() -> (GLint, GLenum) {
		(16, gl::FLOAT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::Mat4(self)
	}
}

impl GLSLType for Dmat2 {
	fn get_type() -> (GLint, GLenum) {
		(4, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat2x3 {
	fn get_type() -> (GLint, GLenum) {
		(6, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat2x4 {
	fn get_type() -> (GLint, GLenum) {
		(8, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat3x2 {
	fn get_type() -> (GLint, GLenum) {
		(6, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat3 {
	fn get_type() -> (GLint, GLenum) {
		(9, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat3x4 {
	fn get_type() -> (GLint, GLenum) {
		(12, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat4x2 {
	fn get_type() -> (GLint, GLenum) {
		(8, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat4x3 {
	fn get_type() -> (GLint, GLenum) {
		(12, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for Dmat4 {
	fn get_type() -> (GLint, GLenum) {
		(16, gl::DOUBLE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for ByteTri {
	fn get_type() -> (GLint, GLenum) {
		(-1, gl::UNSIGNED_BYTE)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}

impl GLSLType for ShortTri {
	fn get_type() -> (GLint, GLenum) {
		(-1, gl::UNSIGNED_SHORT)
	}

	fn into_glsl_any(self) -> GLSLAny {
		GLSLAny::None
	}
}