use gl;
use gl::types::*;

/// Wraps an opengl texture resource, providing a destructor
#[derive(Debug)]
pub struct GLTextureResource {
    handle: GLuint,
}

impl GLTextureResource {
    // Constructs a new handle
    pub fn new() -> Self {
        // create the handle
        let mut handle = 0;
        unsafe {
            gl::GenTextures(1, &mut handle as _);
        }

        // create the object
        Self { handle }
    }

    pub unsafe fn get_raw(&self) -> GLuint {
        self.handle
    }
}

impl Drop for GLTextureResource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.handle as _);
        }
    }
}
