use gl;
use gl::types::*;

// Wraps an opengl shader resource, provhandleing a destructor
pub(super) struct GLShaderResource {
	handle: GLuint
}

impl GLShaderResource {
	pub(super) fn new(shader_type: GLenum) -> Self {
		// check if shader_type is valhandle
		match shader_type {
			gl::VERTEX_SHADER => { },
			gl::GEOMETRY_SHADER => { },
			gl::FRAGMENT_SHADER => { },
			_ => panic!("This shader type is not currently supported!"),
		}

		// create the handle
		let handle = unsafe {
			gl::CreateShader(shader_type)
		};

		// create the object
		Self {
			handle
		}
	}

	pub(super) unsafe fn get_raw(&self) -> GLuint {
		self.handle
	}
}

impl Drop for GLShaderResource {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.handle);
		}
	}
}