use gl;
use gl::types::*;

use super::gl_shader_resource::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::*;
use std::ffi::OsStr;

pub struct Shader {
	pub(super) resource: GLShaderResource,
}

impl Shader {
	pub fn from_file_with_type(path: &Path, shader_type: GLenum) -> Result<Shader, String> {
		// io
		let mut file = match File::open(path) {
			Ok(file) => file,
			Err(_) => return Err(String::from("Could not open file specified")),
		};
		let mut code = String::new();
		file.read_to_string(&mut code).expect("Something went wrong"); // can this happen?

		// get a shader resource
		let resource = GLShaderResource::new(shader_type);

		// upload the code to the gpu
		unsafe {
			gl::ShaderSource(
				resource.get_raw(),
				1,
				&code.as_ptr() as *const *const u8 as _,
				(&(&code).len() as *const usize) as _);
		}

		// compile
		unsafe {
			gl::CompileShader(resource.get_raw());
		}

		// check for errors (this may not be necessary since opengl has an error callback)
		unsafe {
			let mut shader_compiler_success: i32 = 0;
			gl::GetShaderiv(resource.get_raw(), gl::COMPILE_STATUS, &mut shader_compiler_success);
			if shader_compiler_success == gl::FALSE as i32 {
				// get the error message length
				let mut error_length: i32 = 0;
				gl::GetShaderiv(resource.get_raw(), gl::INFO_LOG_LENGTH, &mut error_length);

				// get the error message
				let mut error_log = Vec::<u8>::with_capacity(error_length as usize);
				error_log.set_len(error_length as usize);
				gl::GetShaderInfoLog(
					resource.get_raw(),
					error_length,
					&mut error_length,
					error_log.as_mut_ptr() as _);

                let err_string = format!("{}: {}", path.to_str().unwrap(), String::from_utf8(error_log).unwrap());
                return Err(err_string);

				//return Err(path.to_str().append(String::from_utf8(error_log).unwrap()));
			}
		}

		// return
		Ok(Shader {
			resource,
		})
	}

	pub fn from_file(path: &Path) -> Result<Shader, String> {
		let shader_type = match path.extension().and_then(OsStr::to_str) {
        	Some("geom") => gl::GEOMETRY_SHADER,
        	Some("frag") => gl::FRAGMENT_SHADER,
        	Some("vert") => gl::VERTEX_SHADER,
        	_ => return Err(String::from("Unsupported format")),
    	};
    	Shader::from_file_with_type(path, shader_type)
	}
}
