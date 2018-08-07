use gl;
use gl::types::*;
use std::ffi::CStr;

use super::buffer::*;
use super::glsl_type::*;
use super::shader::*;
use super::draw_env::*;
use super::texture::*;
use super::handles::*;

pub struct DrawEngine {
	// buffer stuff
	buffer_counter: usize,
	buffer_handles: Vec<usize>,
	buffers: Vec<Buffer>,
	buffer_init_lines: Vec<u32>,

	// shader stuff
	shader_counter: usize,
	shader_handles: Vec<usize>,
	shaders: Vec<Shader>,
	shader_init_lines: Vec<u32>,

	// texture stuff
	texture_counter: usize,
	texture_handles: Vec<usize>,
	textures: Vec<Texture2D>,
	texture_init_lines: Vec<u32>,

	// draw command stuff
	draw_env_counter: usize,
	draw_envs: Vec<(f32, usize, DrawEnv, u32)>,
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
			buffers: Vec::new(),
			buffer_init_lines: Vec::new(),

			shader_counter: 0,
			shader_handles: Vec::new(),
			shaders: Vec::new(),
			shader_init_lines: Vec::new(),

			texture_counter: 0,
			texture_handles: Vec::new(),
			textures: Vec::new(),
			texture_init_lines: Vec::new(),

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
	pub fn create_buffer<T>(&mut self, line: u32) -> BufferHandle<T> where T: GLSLType {
		// get a unique handle from the buffer counter
		let buffer_handle = self.buffer_counter;
		self.buffer_counter += 1;

		// create the buffer
		let buffer = Buffer::new::<T>();

		// push
		self.buffer_handles.push(buffer_handle);
		self.buffers.push(buffer);
		self.buffer_init_lines.push(line);

		// create and return the buffer
		BufferHandle::<T>::new(buffer_handle)
	}

	// Frees a buffer
	pub fn free_buffer<T>(&mut self, buffer_handle: BufferHandle<T>) {
		// get the index of the handle
		let index = self.buffer_handles.iter().position(|&handle| handle == buffer_handle.get_id()).expect("Unreachable");	

		// remove
		self.buffer_handles.swap_remove(index);
		self.buffers.swap_remove(index);
		self.buffer_init_lines.swap_remove(index);
	}

	pub fn get_buffer_mut<T, F, R>(&mut self, buffer_handle: &BufferHandle<T>, func: F) -> R where F: Fn(&mut MutableBuffer<T>) -> R {
		// get the index of the handle
		let index = self.buffer_handles.iter().position(|&handle| handle == buffer_handle.get_id()).expect("Unreachable");

		func(&mut self.buffers[index])
	}

	/*pub fn get_buffer<T, F, R>(&self, buffer_handle: BufferHandle<T>, func: F) -> R where F: Fn(&ImmutableBuffer<T>) -> R {
		// get the index of the handle
		let index = self.buffer_handles.iter().position(|&handle| handle == buffer_handle.get_id()).expect("Unreachable");

		//func(&self.buffers[index])

		let b = &mut self.buffers[index] as &mut MutableBuffer<T>;
		let c = b as &ImmutableBuffer<T>;
	}*/

	pub fn create_shader(&mut self, line: u32) -> ShaderHandle {
		// get a unique handle from the shader counter
		let shader_handle = self.shader_counter;
		self.shader_counter += 1;

		// create a shader
		let shader = Shader::new();

		// insert the shader
		self.shader_handles.push(shader_handle);
		self.shaders.push(shader);
		self.shader_init_lines.push(line);

		// return the handle
		ShaderHandle::new(shader_handle)
	}

	pub fn free_shader(&mut self, shader_handle: ShaderHandle) {
		// get the index of the handle
		let index = self.shader_handles.iter().position(|&handle| handle == shader_handle.get_id()).expect("Unreachable");

		// remove
		self.shader_handles.swap_remove(index);
		self.shaders.swap_remove(index);
		self.shader_init_lines.swap_remove(index);
	}

	pub fn get_shader_mut<F, R>(&mut self, shader_handle: &ShaderHandle, func: F) -> R where F: Fn(&mut Shader) -> R {
		// get the index of the handle
		let index = self.shader_handles.iter().position(|&handle| handle == shader_handle.get_id()).expect("Unreachable");

		func(&mut self.shaders[index])
	}

	pub fn create_texture(&mut self, line: u32) -> TextureHandle {
		// get a unique handle from the texture counter
		let texture_handle = self.texture_counter;
		self.texture_counter += 1;

		// create a texture
		let texture = Texture2D::new();

		// insert the texture
		self.texture_handles.push(texture_handle);
		self.textures.push(texture);
		self.texture_init_lines.push(line);

		// return the handle
		TextureHandle::new(texture_handle)
	}

	pub fn free_texture(&mut self, texture_handle: TextureHandle) {
		// get the index of the handle
		let index = self.texture_handles.iter().position(|&handle| handle == texture_handle.get_id()).expect("Unreachable");

		// remove
		self.texture_handles.swap_remove(index);
		self.textures.swap_remove(index);
		self.texture_init_lines.swap_remove(index);
	}

	pub fn get_texture_mut<F, R>(&mut self, texture_handle: &TextureHandle, func: F) -> R where F: Fn(&mut Texture2D) -> R {
		// get the index of the handle
		let index = self.texture_handles.iter().position(|&handle| handle == texture_handle.get_id()).expect("Unreachable");

		func(&mut self.textures[index])
	}

	pub fn create_draw_env(&mut self, depth: f32, line: u32) -> DrawEnvHandle {
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
		self.draw_envs.insert(index, (depth, draw_env_handle, draw_env, line));

		// return the handle
		DrawEnvHandle::new(draw_env_handle)
	}

	pub fn free_draw_env(&mut self, draw_env_handle: &DrawEnvHandle) {
		// get the index of the handle
		let index = self.draw_envs.iter().position(|&(_, h, _, _)| h == draw_env_handle.get_id()).expect("Unreachable");

		// remove
		self.draw_envs.swap_remove(index);
	}

	pub fn get_draw_env<F, R>(&mut self, draw_env_handle: &DrawEnvHandle, func: F) -> R where F: Fn(&mut DrawEnv) -> R {
		// get the index of the handle
		let index = self.draw_envs.iter().position(|&(_, h, _, _)| h == draw_env_handle.get_id()).expect("Unreachable");

		func(&mut self.draw_envs[index].2)
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

		self.draw_envs.iter().for_each(|&(_, _, ref env, line)| {
			// get the program and activate it
			let program_index = self.shader_handles.iter().position(|&handle| handle == env.shader).expect(format!("Program not bound to this draw env ({})", line).as_str());
			let program = &self.shaders[program_index];
			program.activate().expect(format!("Could not activate gl program ({})", self.shader_init_lines[program_index]).as_str());

			// get the index buffer and bind it
			let indices_index = self.buffer_handles.iter().position(|&handle| handle == env.indices).expect(format!("Index buffer not bound to this draw en ({})", line).as_str());
			let indices = &self.buffers[indices_index];
			if indices.buffer_type != gl::ELEMENT_ARRAY_BUFFER {
				panic!(format!("Not a valid buffer_handle ({})", self.buffer_init_lines[indices_index]));
			}

			unsafe {
				gl::BindBuffer(indices.buffer_type, indices.resource.get_handle());
			}

			// bind all the buffers and set up the pointer stuff
			env.buffers.iter().for_each(|&(env_buf_handle, loc)| {
				// get the buffers
				let buffer_index = self.buffer_handles.iter().position(|&handle| handle == env_buf_handle).expect(format!("Not a valid buffer_handle ({})", self.buffer_init_lines[env_buf_handle]).as_str());
				let buffer = &self.buffers[buffer_index];

				// set up the buffer array data
				unsafe {
					gl::BindBuffer(buffer.buffer_type, buffer.resource.get_handle());
					gl::EnableVertexAttribArray(loc);
					gl::VertexAttribPointer(loc, buffer.glsl_type_count, buffer.glsl_type, gl::FALSE, 0, 0u32 as _);
				}
			});

			// textures
			let mut texture_target = 0;
			env.textures.iter().for_each(|&(env_tex_handle, loc)| {
				// get the texture
				let texture_index = self.texture_handles.iter().position(|&handle| handle == env_tex_handle).expect(format!("Not a valid texture_handle ({})", self.texture_init_lines[env_tex_handle]).as_str());
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
			env.uniforms.iter().for_each(|&(ref uniform_data, loc)| {
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