use gl;
use gl::types::*;

use super::gl_shader_resource::*;
use super::gl_program_resource::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::*;

#[derive(Debug)]
pub enum ShaderError {
	// If the program was not linked correctly
	InvalidProgram,
	// If a shader was not built correctly
	InvalidShader,
	// Does not contained the specified uniform name
	NonexistentUniform,
	//
	NonexistentAttrib,
	//
	ShaderBuildFailure,
	//
	ProgramLinkFailure,
}

pub struct Shader {
	program_resource: Option<GLProgramResource>,
	shader_codes: Vec<String>,
	shader_types: Vec<GLenum>,
	shader_resources: Vec<Option<GLShaderResource>>,
}

impl Shader {
	pub(super) fn new() -> Self {
		Self {
			program_resource: None,
			shader_codes: Vec::new(),
			shader_types: Vec::new(),
			shader_resources: Vec::new(),
		}
	}

	pub fn get_attrib_loc(&self, name: &'static str) -> Result<GLint, ShaderError> {
		match self.program_resource {
			Some(ref res) => {
				unsafe {
					let loc = gl::GetAttribLocation(res.get_handle(), name.as_ptr() as *const _ as _);
					if loc == -1 {
						return Err(ShaderError::NonexistentAttrib);
					} else {
						return Ok(loc);
					}
				}
			},
			None => return Err(ShaderError::InvalidProgram),
		}
	}

	pub fn get_uniform_loc(&self, name: &'static str) -> Result<GLint, ShaderError> {
		match self.program_resource {
			Some(ref res) => {
				unsafe {
					let loc = gl::GetUniformLocation(res.get_handle(), name.as_ptr() as *const _ as _);
					if loc == -1 {
						return Err(ShaderError::NonexistentUniform);
					} else {
						return Ok(loc);
					}
				}
			},
			None => return Err(ShaderError::InvalidProgram),
		}
	}

	pub(super) fn activate(&self) -> Result<(), ShaderError> {
		// if the shader is invalid
		if self.program_resource.is_none() {
			return Err(ShaderError::InvalidProgram);
		}

		unsafe {
			gl::UseProgram(self.program_resource.as_ref().unwrap().get_handle());
		}

		Ok(())
	}

	pub fn detach_shader(&mut self, shader_type: GLenum) {
		let index = match self.shader_types.iter().position(|&t| t == shader_type) {
			Some(index) => index,
			None => return,
		};

		self.shader_codes.swap_remove(index);
		self.shader_codes.swap_remove(index);
		self.shader_resources.swap_remove(index);
	}

	pub fn attach_shader(&mut self, shader_code: String, shader_type: GLenum) -> Result<(), ShaderError> {
		// index
		let index = match self.shader_types.iter().position(|&t| t == shader_type) {
			Some(index) => index,
			None => {
				self.shader_codes.push(shader_code);
				self.shader_types.push(shader_type);
				self.shader_resources.push(None);
				self.shader_types.len() - 1
			}
		};

		// create a resource
		let shader_resource = GLShaderResource::new(shader_type);

		// upload the shader code
		unsafe {
			gl::ShaderSource(
				shader_resource.get_handle(),
				1,
				&self.shader_codes[index].as_ptr() as *const *const u8 as _,
				(&(&self.shader_codes[index]).len() as *const usize) as _);
			gl::CompileShader(shader_resource.get_handle());
		}

		// check for errors
		unsafe {
			let mut shader_compile_success: i32 = 0;
            gl::GetShaderiv(shader_resource.get_handle(), gl::COMPILE_STATUS, &mut shader_compile_success);
            if shader_compile_success == gl::FALSE as i32 {
				return Err(ShaderError::ShaderBuildFailure);
			}
		}

		// insert the resource into the vec and return
		self.shader_resources[index] = Some(shader_resource);
		return Ok(());
	}

	// This will probably get removed later
	pub fn attach_shader_from_file(&mut self, path: &Path, shader_type: GLenum) -> Result<(), ShaderError> {
		let mut file = File::open(path).expect("file not found ye");
		let mut code = String::new();
		file.read_to_string(&mut code).expect("sdafadf");
		self.attach_shader(code, shader_type)
	}

	pub fn build(&mut self) -> Result<(), ShaderError> {
		// first, check to see if all resources are Some (are valid)
		if self.shader_resources.iter().any(|res| res.is_none()) {
			return Err(ShaderError::InvalidShader);
		}

		// second, create a program
		let program_resource = GLProgramResource::new();

		// third, attach them all
		self.shader_resources.iter().for_each(|res| unsafe { gl::AttachShader(program_resource.get_handle(), res.as_ref().unwrap().get_handle()); });

		// fourth, link
		unsafe {
			gl::LinkProgram(program_resource.get_handle());
		}

		// fifth, check for errors
		unsafe {
            let mut program_link_success: i32 = 0;
            gl::GetProgramiv(program_resource.get_handle(), gl::LINK_STATUS, &mut program_link_success);
            if program_link_success == gl::FALSE as i32 {
            	return Err(ShaderError::ProgramLinkFailure);
            }	
		}

		// finally, 
		self.program_resource = Some(program_resource);
		Ok(())
	}
}