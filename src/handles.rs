use std::marker::PhantomData;

// Uniform handle
pub struct BufferHandle<T> {
	id: usize,
	phantom: PhantomData<T>,
}

impl<T> BufferHandle<T> {
	pub(super) fn new(id: usize) -> Self {
		Self {
			id,
			phantom: PhantomData,
		}
	}

	pub(super) fn get_id(&self) -> usize {
		self.id
	}
}

// Texture handle
pub struct TextureHandle {
	id: usize,
}

impl TextureHandle {
	pub(super) fn new(id: usize) -> Self {
		Self {
			id,
		}
	}

	pub(super) fn get_id(&self) -> usize {
		self.id
	}
}

// Shader handle
pub struct ShaderHandle {
	id: usize,
}

impl ShaderHandle {
	pub(super) fn new(id: usize) -> Self {
		Self {
			id,
		}
	}

	pub(super) fn get_id(&self) -> usize {
		self.id
	}
}

// Draw env handle
pub struct DrawEnvHandle {
	id: usize,
}

impl DrawEnvHandle {
	pub(super) fn new(id: usize) -> Self {
		Self {
			id,
		}
	}

	pub(super) fn get_id(&self) -> usize {
		self.id
	}
}