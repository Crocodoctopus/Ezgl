use gl;
use gl::types::*;

use std;
use super::gl_buffer_resource::*;
use super::glsl_type::*;

#[derive(Debug)]
pub enum BufferError {
	InvalidBufferType,
	BufferNotInitialized,
	InvalidGLSLType
}

pub struct Buffer {
	//data: Vec<T>,
	pub(super) buffer_type: GLenum,
	pub(super) glsl_type_count: GLint,
	pub(super) glsl_type: GLenum,
	pub(super) resource: GLBufferResource,
}

impl Buffer {
	pub(super) fn new<T>() -> Self where T: GLSLType {
		Self {
			//data: Vec::new(),
			buffer_type: 0,
			glsl_type_count: T::get_type().0,
			glsl_type: T::get_type().1,
			resource: GLBufferResource::new(),
		}
	}
}

// mutable functions
pub trait MutableBuffer<T> {
	fn init(&mut self, GLenum, Box<[T]>) -> Result<(), BufferError>;
	fn blit(&mut self, usize, &[T]) -> Result<(), BufferError> where T: Copy;
}

// immutable functions
/*pub trait ImmutableBuffer<T> {
	fn get(&self);
}

impl<T> ImmutableBuffer<T> for MutableBuffer<T> {
	fn get(&self) {
		
	}
}*/

impl<T> MutableBuffer<T> for Buffer {
	fn init(&mut self, buffer_type: GLenum, data: Box<[T]>) -> Result<(), BufferError> {
		// check for vailidity
		match buffer_type {
			gl::ARRAY_BUFFER => {
				match self.glsl_type {
					gl::UNSIGNED_BYTE | gl::UNSIGNED_SHORT => return Err(BufferError::InvalidGLSLType),
					_ => { },
				}
			},
			gl::ELEMENT_ARRAY_BUFFER => {
				match self.glsl_type {
					gl::UNSIGNED_BYTE | gl::UNSIGNED_SHORT => { },
					_ => return Err(BufferError::InvalidGLSLType),
				}
			},
			_=> return Err(BufferError::InvalidBufferType),
		}

		// 
		//self.data = data.into_vec();

		// upload the data
		unsafe {
			gl::BindBuffer(buffer_type, self.resource.get_handle());
			gl::BufferData(
				buffer_type, 
				(data.len() * std::mem::size_of::<T>()) as _,
				data.as_ptr() as _,
				gl::STATIC_DRAW);
		}

		self.buffer_type = buffer_type;

		Ok(())
	}

	fn blit(&mut self, pos: usize, data: &[T]) -> Result<(), BufferError> where T: Copy {
		// check for vailidity
		match self.buffer_type {
			0 => return Err(BufferError::BufferNotInitialized),
			_=> { },
		}

		// upload the data
		unsafe {
			gl::BindBuffer(self.buffer_type, self.resource.get_handle());
			gl::BufferSubData(
				self.buffer_type,
				(pos * std::mem::size_of::<T>()) as _,
				(data.len() * std::mem::size_of::<T>()) as _,
				data.as_ptr() as _);
		}

		Ok(())
		// copy the data into the buffer's local data
		//self.data.splice(pos .. pos + data.len(), data.into_iter().map(|x| *x));
	}
}