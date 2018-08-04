use gl;
use gl::types::*;

use std;
use super::gl_buffer_resource::*;

pub struct Buffer<T> {
	data: Vec<T>,
	buffer_type: GLenum,
	buffer_key: usize,
	resource: Option<GLBufferResource>,
}

impl<T> Buffer<T> {
	pub(super) fn new(buffer_type: GLenum, buffer_key: usize) -> Self {
		Self {
			data: Vec::new(),
			buffer_type,
			buffer_key,
			resource: None,
		}
	}

	pub fn init(&mut self, data: Box<[T]>) {
		// check if we have the resource
		let resource = self.resource.as_ref().expect("Buffer: Invalid resource!");

		// 
		self.data = data.into_vec();

		// upload the data
		unsafe {
			gl::BindBuffer(self.buffer_type, resource.get_handle());
			gl::BufferData(
				self.buffer_type, 
				(self.data.len() * std::mem::size_of::<T>()) as _,
				self.data.as_ptr() as _,
				gl::STATIC_DRAW);
		}
	}

	pub fn blit(&mut self, pos: usize, data: &[T]) where T: Copy {
		// check if we have the resource
		let resource = self.resource.as_ref().expect("Buffer: Invalid resource!");

		// upload the data
		unsafe {
			gl::BindBuffer(self.buffer_type, resource.get_handle());
			gl::BufferSubData(
				self.buffer_type,
				(pos * std::mem::size_of::<T>()) as _,
				(data.len() * std::mem::size_of::<T>()) as _,
				data.as_ptr() as _);
		}

		// copy the data into the buffer's local data
		self.data.splice(pos .. pos + data.len(), data.into_iter().map(|x| *x));
	}

	//
	pub(super) fn get_key(&self) -> usize {
		self.buffer_key
	}

	//
	pub(super) fn get_resource_mut(&mut self) -> &mut Option<GLBufferResource> {
		&mut self.resource
	}
}