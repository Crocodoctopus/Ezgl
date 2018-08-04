use gl::types::*;
use super::gl_buffer_resource::*;

// Contains data about a buffer
pub(super) struct BufferData {
	pub(super) buffer_type: GLenum,
	pub(super) glsl_type_count: GLint,
	pub(super) glsl_type: GLenum,
	pub(super) resource: Option<GLBufferResource>,
}