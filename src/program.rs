use gl;
use gl::types::*;

use super::shader::*;
use super::gl_program_resource::*;

pub struct ProgramBuilder {
	shaders: Vec<Shader>,
}

impl ProgramBuilder {
	pub fn new() -> Self {
		Self {
			shaders: Vec::new(),
		}
	}

	pub fn with(mut self, shader: Shader) -> Self {
		self.shaders.push(shader);
		self
	}

	pub fn build(mut self) -> Result<Program, String> {
		let resource = GLProgramResource::new();

		self.shaders.iter().for_each(|shader| {
			unsafe {
				gl::AttachShader(resource.get_raw(), shader.resource.get_raw());
			}
		});

		unsafe {
			gl::LinkProgram(resource.get_raw());
		}

		unsafe {
            let mut program_link_success: i32 = 0;
            gl::GetProgramiv(resource.get_raw(), gl::LINK_STATUS, &mut program_link_success);
            if program_link_success == gl::FALSE as i32 {
            	return Err(String::from("Link error"));
            }	
		}

		Ok(Program {
			resource,
		})
	}
}

pub struct Program {
	pub(super) resource: GLProgramResource,
}