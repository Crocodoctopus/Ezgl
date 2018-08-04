use gl;
use gl::types::*;
use std::mem::swap;
use std::ffi::CStr;

use super::buffer_data::*;
use super::gl_buffer_resource::*;
use super::buffer::*;
use super::glsl_type::*;
use super::shader::*;
use super::draw_env::*;
use super::texture::*;
use super::handles::*;

pub enum DrawEngineError {
	InvalidHandle,
}

pub struct DrawEngine {
	// buffer stuff
	buffer_counter: usize,
	buffer_handles: Vec<usize>,
	buffer_data: Vec<BufferData>,
	buffer_names: Vec<&'static str>,

	// shader stuff
	shader_counter: usize,
	shader_handles: Vec<usize>,
	shaders: Vec<Shader>,
	shader_names: Vec<&'static str>,

	// texture stuff
	texture_counter: usize,
	texture_handles: Vec<usize>,
	textures: Vec<Texture2D>,
	texture_names: Vec<&'static str>,

	// uniform stuff
	uniform_counter: usize,
	uniform_handles: Vec<usize>,
	uniform_data: Vec<GLSLAny>,
	uniform_names: Vec<&'static str>,

	// draw command stuff
	draw_env_counter: usize,
	draw_envs: Vec<(f32, usize, DrawEnv, &'static str)>,
}

extern "system" fn callback(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, _length: GLsizei, message: *const GLchar, _: *mut GLvoid) {
	unsafe {
		let rust_message = CStr::from_ptr(message).to_str().unwrap().to_owned();
		println!("A GL error has been thrown!");
		println!("  source: {:?}, type: {:?}, id: {:?}, severity: {:?}", source, gltype, id, severity);
		println!("  Message: {:?}", rust_message);
	}
}

impl DrawEngine {
	pub fn new() -> Self {
		// TODO: create a VAO(?)
		Self {
			buffer_counter: 0,
			buffer_handles: Vec::new(),
			buffer_data: Vec::new(),
			buffer_names: Vec::new(),

			shader_counter: 0,
			shader_handles: Vec::new(),
			shaders: Vec::new(),
			shader_names: Vec::new(),

			texture_counter: 0,
			texture_handles: Vec::new(),
			textures: Vec::new(),
			texture_names: Vec::new(),

			uniform_counter: 0,
			uniform_handles: Vec::new(),
			uniform_data: Vec::new(),
			uniform_names: Vec::new(),

			draw_env_counter: 0,
			draw_envs: Vec::new(),
		}
	}

	pub fn enable_debug() {
		unsafe {
			gl::Enable(gl::DEBUG_OUTPUT);
			gl::DebugMessageCallback(callback, 0 as _);
		}
	}

	// Creates a new buffer with nothing in it
	pub fn create_buffer<T>(&mut self, buffer_type: GLenum, name: &'static str) -> Buffer<T> where T: GLSLType {
		// do some quick support checks
		match buffer_type {
			gl::ARRAY_BUFFER => { },
			gl::ELEMENT_ARRAY_BUFFER => panic!("Use create_element_buffer instead!"),
			_ => panic!("This buffer type is not supported"),
		}

		// get a unique handle from the buffer counter
		let buffer_handle = self.buffer_counter;
		self.buffer_counter += 1;

		// create a buffer resource
		let gl_buffer_resource = GLBufferResource::new();

		// create the buffer data
		let buffer_data = BufferData {
			buffer_type,
			glsl_type_count: T::get_type().0,
			glsl_type: T::get_type().1,
			resource: Some(gl_buffer_resource),
		};

		// push
		self.buffer_handles.push(buffer_handle);
		self.buffer_data.push(buffer_data);
		self.buffer_names.push(name);

		// create and return the buffer
		Buffer::<T>::new(buffer_type, buffer_handle)
	}

	pub fn create_element_buffer<T>(&mut self, name: &'static str) -> Buffer<T> where T: ElementType {
		// get a unique handle from the buffer counter
		let buffer_handle = self.buffer_counter;
		self.buffer_counter += 1;

		// create a buffer resource
		let gl_buffer_resource = GLBufferResource::new();

		// create the buffer data
		let buffer_data = BufferData {
			buffer_type: gl::ELEMENT_ARRAY_BUFFER,
			glsl_type_count: 0,
			glsl_type: T::get_type(),
			resource: Some(gl_buffer_resource),
		};

		// push
		self.buffer_handles.push(buffer_handle);
		self.buffer_data.push(buffer_data);
		self.buffer_names.push(name);

		// create and return the buffer
		Buffer::<T>::new(gl::ELEMENT_ARRAY_BUFFER, buffer_handle)
	}

	// Frees a buffer
	pub fn free_buffer<T>(&mut self, buffer: Buffer<T>) {
		// get the index of the handle
		let index = self.buffer_handles.iter().position(|&handle| handle == buffer.get_key()).expect("I'm not sure how this can happen");	

		// remove
		self.buffer_handles.swap_remove(index);
		self.buffer_data.swap_remove(index);
		self.buffer_names.swap_remove(index);
	}

	// Swaps the important buffer resource
	pub fn buffer_swap<T>(&mut self, buffer: &mut Buffer<T>) {
		// get the index of the handle
		let index = self.buffer_handles.iter().position(|&handle| handle == buffer.get_key()).expect("I'm not sure how this can happen");

		// Swap the resources
		swap(buffer.get_resource_mut(), &mut self.buffer_data[index].resource);
	}

	pub fn create_shader(&mut self, name: &'static str) -> ShaderHandle {
		// get a unique handle from the shader counter
		let shader_handle = self.shader_counter;
		self.shader_counter += 1;

		// create a shader
		let shader = Shader::new();

		// insert the shader
		self.shader_handles.push(shader_handle);
		self.shaders.push(shader);
		self.shader_names.push(name);

		// return the handle
		ShaderHandle::new(shader_handle)
	}

	pub fn free_shader(&mut self, shader_handle: ShaderHandle) -> Result<(), DrawEngineError> {
		// get the index of the handle
		let index = match self.shader_handles.iter().position(|&handle| handle == shader_handle.get_id()) {
			Some(index) => index,
			None => return Err(DrawEngineError::InvalidHandle),
		};

		// remove
		self.shader_handles.swap_remove(index);
		self.shaders.swap_remove(index);
		self.shader_names.swap_remove(index);

		Ok(())
	}

	pub fn get_shader(&mut self, shader_handle: &ShaderHandle) -> Option<&mut Shader> {
		match self.shader_handles.iter().position(|&handle| handle == shader_handle.get_id()) {
			Some(index) => Some(&mut self.shaders[index]),
			None => None,
		}
	}

	pub fn create_texture(&mut self, name: &'static str) -> TextureHandle {
		// get a unique handle from the texture counter
		let texture_handle = self.texture_counter;
		self.texture_counter += 1;

		// create a texture
		let texture = Texture2D::new();

		// insert the texture
		self.texture_handles.push(texture_handle);
		self.textures.push(texture);
		self.texture_names.push(name);

		// return the handle
		TextureHandle::new(texture_handle)
	}

	pub fn free_texture(&mut self, texture_handle: TextureHandle) -> Result<(), DrawEngineError> {
		// get the index of the handle
		let index = match self.texture_handles.iter().position(|&handle| handle == texture_handle.get_id()) {
			Some(index) => index,
			None => return Err(DrawEngineError::InvalidHandle),
		};

		// remove
		self.texture_handles.swap_remove(index);
		self.textures.swap_remove(index);
		self.texture_names.swap_remove(index);

		Ok(())
	}

	pub fn get_texture(&mut self, texture_handle: &TextureHandle) -> Option<&mut Texture2D> {
		match self.texture_handles.iter().position(|&handle| handle == texture_handle.get_id()) {
			Some(index) => Some(&mut self.textures[index]),
			None => None,
		}
	}

	pub fn create_draw_env(&mut self, depth: f32, name: &'static str) -> usize {
		// get a unique handle from the draw env counter
		let draw_env_handle = self.draw_env_counter;
		self.draw_env_counter += 1;

		// create a new draw env
		let draw_env = DrawEnv::new();

		// sorted insert
		let index = match self.draw_envs.iter().position(|&(d, _, _, _)| d > depth) {
			Some(index) => index,
			None => self.draw_envs.len(),
		};
		self.draw_envs.insert(index, (depth, draw_env_handle, draw_env, name));

		// return the handle
		draw_env_handle
	}

	pub fn free_draw_env(&mut self, draw_env_handle: usize) -> Result<(), DrawEngineError> {
		// get the index of the handle
		let index = match self.draw_envs.iter().position(|&(_, h, _, _)| h == draw_env_handle) {
			Some(index) => index,
			None => return Err(DrawEngineError::InvalidHandle),
		};

		// remove
		self.draw_envs.swap_remove(index);

		Ok(())
	}

	pub fn get_draw_env(&mut self, draw_env_handle: usize) -> Option<&mut DrawEnv> {
		match self.draw_envs.iter().position(|&(_, h, _, _)| h == draw_env_handle) {
			Some(index) => Some(&mut self.draw_envs[index].2),
			None => None,
		}
	}

	pub fn create_uniform<T>(&mut self, t: T, name: &'static str) -> UniformHandle<T> where T: GLSLType {
		// check if this value of t is supported, and extract the "any"
		let data = match t.into_glsl_any() {
			GLSLAny::None => panic!("Opengl does not support this kind of uniform"),
			a => a,
		};

		// get a unique handle from the uniform counter
		let uniform_handle = self.uniform_counter;
		self.uniform_counter += 1;

		// insert
		self.uniform_handles.push(uniform_handle);
		self.uniform_data.push(data);
		self.uniform_names.push(name);

		// return the handle
		UniformHandle::new(uniform_handle)
	}

	pub fn set_uniform<T: 'static>(&mut self, handle: &UniformHandle<T>, t: T) where T: GLSLType {
		// get the index of the uniform
		let index = match self.draw_envs.iter().position(|&(_, h, _, _)| h == handle.get_id()) {
			Some(index) => index,
			None => unreachable!(),
		};

		// capture the data
		self.uniform_data[index] = t.into_glsl_any();
	}

	pub fn free_uniform<T>(&mut self, handle: UniformHandle<T>) {
		// get the index of the uniform
		let index = match self.draw_envs.iter().position(|&(_, h, _, _)| h == handle.get_id()) {
			Some(index) => index,
			None => unreachable!(),
		};

		// remove
		self.uniform_handles.swap_remove(index);
		self.uniform_data.swap_remove(index);
		self.uniform_names.swap_remove(index);
	}

	// THE BIG DRAW
	pub fn draw(&self, count: GLint, offset: GLint) {
    	unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		let mut vao = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut vao as _);
			gl::BindVertexArray(vao);
		}

		self.draw_envs.iter().for_each(|&(_, _, ref env, name)| {
			// get the program and activate it
			let program_index = self.shader_handles.iter().position(|&handle| handle == env.shader).expect(format!("Program not bound to this draw env ({})", name).as_str());
			let program = &self.shaders[program_index];
			program.activate().expect(format!("Could not activate gl program ({})", self.shader_names[program_index]).as_str());

			// get the index buffer and bind it
			let indices_index = self.buffer_handles.iter().position(|&handle| handle == env.indices).expect(format!("Index buffer not bound to this draw en ({})", name).as_str());
			let indices = &self.buffer_data[indices_index];
			if indices.buffer_type != gl::ELEMENT_ARRAY_BUFFER {
				panic!(format!("Not a valid buffer_handle ({})", self.buffer_names[indices_index]));
			}

			unsafe {
				gl::BindBuffer(indices.buffer_type, indices.resource.as_ref().expect(format!("This buffer is not owned by the draw engine ({})", self.buffer_names[indices_index]).as_str()).get_handle());
			}

			// bind all the buffers and set up the pointer stuff
			env.buffers.iter().for_each(|&(env_buf_handle, loc)| {
				// get the buffers
				let buffer_index = self.buffer_handles.iter().position(|&handle| handle == env_buf_handle).expect(format!("Not a valid buffer_handle ({})", self.buffer_names[env_buf_handle]).as_str());
				let buffer = &self.buffer_data[buffer_index];

				// set up the buffer array data
				unsafe {
					gl::BindBuffer(buffer.buffer_type, buffer.resource.as_ref().expect(format!("This buffer is not owned by the draw engine ({})", self.buffer_names[indices_index]).as_str()).get_handle());
					gl::EnableVertexAttribArray(loc);
					gl::VertexAttribPointer(loc, buffer.glsl_type_count, buffer.glsl_type, gl::FALSE, 0, 0u32 as _);
				}
			});

			// textures
			let mut texture_target = 0;
			env.textures.iter().for_each(|&(env_tex_handle, loc)| {
				// get the texture
				let texture_index = self.texture_handles.iter().position(|&handle| handle == env_tex_handle).expect(format!("Not a valid texture_handle ({})", self.texture_names[env_tex_handle]).as_str());
				let texture = &self.textures[texture_index];

				// uniform magic aaaay
				unsafe {
					gl::ActiveTexture(gl::TEXTURE0 + texture_target);
            		gl::BindTexture(gl::TEXTURE_2D, texture.resource.get_handle());
            		gl::Uniform1i(loc, texture_target as _);
				}

				// increment the texture_target handle
				texture_target += 1;
			});

			// depth
			match env.depth {
				EnableDepth::No => {
					unsafe {
						gl::Disable(gl::DEPTH_TEST);
					}
				}
				EnableDepth::Yes(arg1) => {
					unsafe {
						gl::Enable(gl::DEPTH_TEST);
						gl::DepthFunc(arg1);
					}
				}
			}

			// blend
			match env.blend {
				EnableBlend::No => {
					unsafe {
						gl::Disable(gl::BLEND);
					}
				}
				EnableBlend::Yes(arg1, arg2) => {
					unsafe {
						gl::Enable(gl::BLEND);
						gl::BlendFunc(arg1, arg2);
					}
				}
			}

			// Uniforms
			env.uniforms.iter().for_each(|&(env_unif_handle, loc)| {
				// get the uniform data
				let uniform_index = self.uniform_handles.iter().position(|&handle| handle == env_unif_handle).expect(format!("Not a valid uniform_handle ({})", self.uniform_names[env_unif_handle]).as_str());
				let uniform_data = &self.uniform_data[uniform_index];

				// make the appropriate opengl uniform call
				unsafe {
					match uniform_data {
						GLSLAny::Float(float) => gl::Uniform1f(loc, float.0),
						GLSLAny::Vec2(vec2) => gl::Uniform2f(loc, vec2.0, vec2.1),
						GLSLAny::Vec3(vec3) => gl::Uniform3f(loc, vec3.0, vec3.1, vec3.2),
						GLSLAny::Vec4(vec4) => gl::Uniform4f(loc, vec4.0, vec4.1, vec4.2, vec4.3),
						GLSLAny::Int(int) => gl::Uniform1i(loc, int.0),
						GLSLAny::Ivec2(ivec2) => gl::Uniform2i(loc, ivec2.0, ivec2.1),
						GLSLAny::Ivec3(ivec3) => gl::Uniform3i(loc, ivec3.0, ivec3.1, ivec3.2),
						GLSLAny::Ivec4(ivec4) => gl::Uniform4i(loc, ivec4.0, ivec4.1, ivec4.2, ivec4.3),
						GLSLAny::Uint(uint) => gl::Uniform1ui(loc, uint.0),
						GLSLAny::Uvec2(uvec2) => gl::Uniform2ui(loc, uvec2.0, uvec2.1),
						GLSLAny::Uvec3(uvec3) => gl::Uniform3ui(loc, uvec3.0, uvec3.1, uvec3.2),
						GLSLAny::Uvec4(uvec4) => gl::Uniform4ui(loc, uvec4.0, uvec4.1, uvec4.2, uvec4.3),
						GLSLAny::Bool(glbool) => gl::Uniform1ui(loc, glbool.0 as _),  
						GLSLAny::Bvec2(bvec2) => gl::Uniform2ui(loc, bvec2.0 as _, bvec2.1 as _), 
						GLSLAny::Bvec3(bvec3) => gl::Uniform3ui(loc, bvec3.0 as _, bvec3.1 as _, bvec3.2 as _), 
						GLSLAny::Bvec4(bvec4) => gl::Uniform4ui(loc, bvec4.0 as _, bvec4.1 as _, bvec4.2 as _, bvec4.3 as _),
						GLSLAny::Mat2(mat2) => gl::UniformMatrix2fv(loc, 1, gl::FALSE, mat2 as *const _ as _),
						GLSLAny::Mat3(mat3) => gl::UniformMatrix3fv(loc, 1, gl::FALSE, mat3 as *const _ as _),
						GLSLAny::Mat4(mat4) => gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat4 as *const _ as _),
						GLSLAny::Mat2x3(mat2x3) => gl::UniformMatrix2x3fv(loc, 1, gl::FALSE, mat2x3 as *const _ as _),
						GLSLAny::Mat3x2(mat3x2) => gl::UniformMatrix3x2fv(loc, 1, gl::FALSE, mat3x2 as *const _ as _),
						GLSLAny::Mat2x4(mat2x4) => gl::UniformMatrix2x4fv(loc, 1, gl::FALSE, mat2x4 as *const _ as _),
						GLSLAny::Mat4x2(mat4x2) => gl::UniformMatrix4x2fv(loc, 1, gl::FALSE, mat4x2 as *const _ as _),
						GLSLAny::Mat3x4(mat3x4) => gl::UniformMatrix3x4fv(loc, 1, gl::FALSE, mat3x4 as *const _ as _),
						GLSLAny::Mat4x3(mat4x3) => gl::UniformMatrix4x3fv(loc, 1, gl::FALSE, mat4x3 as *const _ as _),
						GLSLAny::None => unreachable!(),
					}
				}
			});

			// draw
			unsafe {
				gl::DrawElements(
					gl::TRIANGLES,
					(count * 3) as _,
					indices.glsl_type,
					offset as _);
			}
		});

		unsafe {
			gl::DeleteVertexArrays(1, &mut vao as _);
			assert!(gl::GetError() == gl::NO_ERROR);
		}
	}
}