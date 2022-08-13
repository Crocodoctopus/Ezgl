use gl;
use gl::types::*;

use png;
use std::fs::File;
use std::path::*;

use super::gl_texture_resource::*;

#[derive(Debug)]
pub enum Texture2DError {
    FileNotFound,
    InvalidDataDimensions,
    OutOfBounds,
    FormatNotSupported,
}

#[derive(Debug)]
pub struct Texture2D {
    pub width: u32,
    pub height: u32,
    pub format: GLenum,
    pub(super) resource: GLTextureResource,
}

impl Texture2D {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            format: 0,
            resource: GLTextureResource::new(),
        }
    }

    pub fn load_from_file(&mut self, path: &Path) -> Result<(), Texture2DError> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(Texture2DError::FileNotFound),
        };

        let png_decoder = png::Decoder::new(file);
        let (info, mut reader) = match png_decoder.read_info() {
            Ok((info, reader)) => (info, reader),
            Err(_) => return Err(Texture2DError::FormatNotSupported),
        };

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let format = match info.color_type {
            png::ColorType::RGB => gl::RGB,
            png::ColorType::RGBA => gl::RGBA,
            _ => return Err(Texture2DError::FormatNotSupported),
        };

        self.load_from_pixels(info.width, info.height, format, &buf)
    }

    pub fn load_from_pixels(
        &mut self,
        width: u32,
        height: u32,
        format: GLenum,
        data: &[u8],
    ) -> Result<(), Texture2DError> {
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
            gl::BindTexture(gl::TEXTURE_2D, self.resource.get_raw());
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as _,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
        }

        // Set state
        self.width = width;
        self.height = height;
        self.format = format;

        //
        Ok(())
    }

    /// Blits a chunk of data to a region of a Texture2D object
    pub fn blit(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        data: Box<[u8]>,
    ) -> Result<(), Texture2DError> {
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
            gl::BindTexture(gl::TEXTURE_2D, self.resource.get_raw());
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                x as _,
                y as _,
                width as _,
                height as _,
                self.format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as _,
            );
        };

        // return success
        Ok(())
    }
}
