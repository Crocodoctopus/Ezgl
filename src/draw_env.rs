use gl::types::*;

use super::buffer::*;
use super::handles::*;

pub(super) enum EnableDepth {
	No,
	Yes(GLenum),
}

pub(super) enum EnableBlend {
	No,
	Yes(GLenum, GLenum),
}

// :)
pub struct DrawEnv {
	// core stuff
	pub(super) shader: usize,
	pub(super) indices: usize,
	pub(super) buffers: Vec<(usize, GLuint)>,

	// optional stuff
	pub(super) depth: EnableDepth,
	pub(super) blend: EnableBlend,

	// textures
	pub(super) textures: Vec<(usize, GLint)>,

	// uniforms
	pub(super) uniforms: Vec<(usize, GLint)>,
}

impl DrawEnv {
	pub(super) fn new() -> Self {
		Self {
			shader: usize::max_value(),
			indices: usize::max_value(),
			buffers: Vec::new(),

			depth: EnableDepth::No,
			blend: EnableBlend::No,

			textures: Vec::new(),

			uniforms: Vec::new(),
		}
	}

	pub fn add_shader(&mut self, shader_handle: &ShaderHandle) {
		self.shader = shader_handle.get_id();
	}

	pub fn add_index_buffer<T>(&mut self, index_buffer: &Buffer<T>) {
		self.indices = index_buffer.get_key();
	}

	pub fn add_buffer<T>(&mut self, buffer: &Buffer<T>, attrib_loc: GLuint) {
		// push only if the handle isn't already there
		if !self.buffers.iter().any(|&(h, _)| h == buffer.get_key()) {
			self.buffers.push((buffer.get_key(), attrib_loc));
		}
	}

	pub fn remove_buffer<T>(&mut self, buffer: &Buffer<T>) {
		// find the index (or return)
		let index = match self.buffers.iter().position(|&(h, _)| h == buffer.get_key()) {
			Some(index) => index,
			None => return,
		};
		
		// remove at said index
		self.buffers.swap_remove(index);
	}

	pub fn add_texture(&mut self, texture_handle: &TextureHandle, attrib_loc: GLuint) {
		// push only if the handle isn't already there
		if !self.buffers.iter().any(|&(h, _)| h == texture_handle.get_id()) {
			self.buffers.push((texture_handle.get_id(), attrib_loc));
		}
	}

	pub fn remove_texture(&mut self, texture_handle: &TextureHandle) {
		// find the index (or return)
		let index = match self.buffers.iter().position(|&(h, _)| h == texture_handle.get_id()) {
			Some(index) => index,
			None => return,
		};
		
		// remove at said index
		self.buffers.swap_remove(index);
	}

	pub fn add_uniform<T>(&mut self, uniform: &UniformHandle<T>, attrib_loc: GLint) {
		// push only if the handle isn't already there
		if !self.uniforms.iter().any(|&(h, _)| h == uniform.get_id()) {
			self.uniforms.push((uniform.get_id(), attrib_loc));
		}
	}

	pub fn remove_uniform<T>(&mut self, uniform: &UniformHandle<T>) {
		// find the index (or return)
		let index = match self.uniforms.iter().position(|&(h, _)| h == uniform.get_id()) {
			Some(index) => index,
			None => return,
		};
		
		// remove at said index
		self.uniforms.swap_remove(index);
	}

	pub fn enable_depth(&mut self, arg1: GLenum) {
		self.depth = EnableDepth::Yes(arg1);
	}

	pub fn disable_depth(&mut self) {
		self.depth = EnableDepth::No;
	}

	pub fn enable_blend(&mut self, arg1: GLenum, arg2: GLenum) {
		self.blend = EnableBlend::Yes(arg1, arg2);
	}

	pub fn disable_blend(&mut self) {
		self.blend = EnableBlend::No;
	}
}