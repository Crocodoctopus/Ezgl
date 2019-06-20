use gl;
use gl::types::*;

// Wraps an opengl program resource, provhandleing a destructor
pub(super) struct GLProgramResource {
    handle: GLuint,
}

impl GLProgramResource {
    pub(super) fn new() -> Self {
        // create the handle
        let handle = unsafe { gl::CreateProgram() };

        // create the object
        Self { handle }
    }

    pub(super) unsafe fn get_raw(&self) -> GLuint {
        self.handle
    }
}

impl Drop for GLProgramResource {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.handle);
        }
    }
}
