use gl;
use gl::types::*;

use super::gl_texture_resource::*;

#[derive(Debug)]
pub enum Texture2DError {
	InvalidDataDimensions,
    OutOfBounds,
    FormatNotSupported,
}

pub struct Texture2D {
	width: u32,
	height: u32,
	format: GLenum,
	pub(super) resource: GLTextureResource,
}

impl Texture2D {
	pub(super) fn new() -> Self {
		Self {
			width: 0,
			height: 0,
			format: 0,
			resource: GLTextureResource::new(),
		}
	}

	pub fn init(&mut self, width: u32, height: u32, format: GLenum, data: Box<[u8]>) -> Result<(), Texture2DError> {
        // get the number of bytes per color
        let bytes_per_color = match format {
            gl::RGB => 3,
            gl::RGBA => 4,
            _ => return Err(Texture2DError::FormatNotSupported),
        };

        // check if the data fits the regions
        if bytes_per_color * width * height != data.len() as u32 {
        	return Err(Texture2DError::InvalidDataDimensions);
        }

        // upload the data
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.resource.get_handle());
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as _,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _);
           gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
           gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
           gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
           gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        }

        // set some state
        self.width = width;
        self.height = height;
        self.format = format;

        // ret
        Ok(())
	}

    /// Blits a chunk of data to a region of a Texture2D object
    pub fn blit(&mut self, x: u32, y: u32, width: u32, height: u32, data: Box<[u8]>) -> Result<(), Texture2DError> {
        // check if we're blitting out of bounds
        if x + width > self.width || y + height > self.height {
            return Err(Texture2DError::OutOfBounds);
        }

        // get the number of bytes per color
        let bytes_per_color = match self.format {
            gl::RGB => 3,
            gl::RGBA => 4,
            _ => return Err(Texture2DError::FormatNotSupported),
        };

        // check if the data fits the regions
        if bytes_per_color * width * height != data.len() as u32 {
        	return Err(Texture2DError::InvalidDataDimensions);
        }

        // blit
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.resource.get_handle());
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                x as _,
                y as _,
                width as _,
                height as _,
                self.format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _);
        };

        // return success
        Ok(())
    }
}