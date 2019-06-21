use gl;
use gl::types::*;

// Wraps an opengl buffer resource, providing a destructor
pub(super) struct GLBufferResource {
    handle: GLuint,
}

impl GLBufferResource {
    pub(super) fn new() -> Self {
        // create the handle
        let mut handle = 0;
        unsafe {
            gl::GenBuffers(1, &mut handle as _);
        }

        // create the object
        Self { handle }
    }

    pub(super) unsafe fn get_raw(&self) -> GLuint {
        self.handle
    }
}

impl Drop for GLBufferResource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.handle as _);
        }
    }
}
