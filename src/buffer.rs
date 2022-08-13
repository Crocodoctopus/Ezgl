use gl;
use gl::types::*;

use std::marker::PhantomData;

use super::gl_buffer_resource::*;
use std;

#[derive(Debug)]
pub enum BufferError {
    BufferNotInitialized,
}

pub struct Buffer<T> {
    phantom: PhantomData<T>,
    pub(super) buffer_type: GLenum,
    pub(super) resource: GLBufferResource,
}

impl<T> Buffer<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            buffer_type: 0,
            resource: GLBufferResource::new(),
        }
    }

    pub fn from(buffer_type: GLenum, data: &[T]) -> Self {
        // init
        let s = Self {
            phantom: PhantomData,
            buffer_type,
            resource: GLBufferResource::new(),
        };

        // upload the data
        unsafe {
            gl::BindBuffer(buffer_type, s.resource.get_raw());
            gl::BufferData(
                buffer_type,
                (data.len() * std::mem::size_of::<T>()) as _,
                data.as_ptr() as _,
                gl::STATIC_DRAW,
            );
        }

        s
    }

    pub fn init(&mut self, buffer_type: GLenum, data: &[T]) -> Result<(), BufferError> {
        // upload the data
        unsafe {
            gl::BindBuffer(buffer_type, self.resource.get_raw());
            gl::BufferData(
                buffer_type,
                (data.len() * std::mem::size_of::<T>()) as _,
                data.as_ptr() as _,
                gl::STATIC_DRAW,
            );
        }

        self.buffer_type = buffer_type;

        Ok(())
    }

    pub fn init_null(&mut self, buffer_type: GLenum, len: usize) -> Result<(), BufferError> {
        // upload the data
        unsafe {
            gl::BindBuffer(buffer_type, self.resource.get_raw());
            gl::BufferData(
                buffer_type,
                (len * std::mem::size_of::<T>()) as _,
                0 as _,
                gl::STATIC_DRAW,
            );
        }

        self.buffer_type = buffer_type;

        Ok(())
    }

    pub fn splice(&mut self, pos: usize, data: &[T]) -> Result<(), BufferError>
    where
        T: Copy,
    {
        // check for vailidity
        match self.buffer_type {
            0 => return Err(BufferError::BufferNotInitialized),
            _ => {}
        }

        // upload the data
        unsafe {
            gl::BindBuffer(self.buffer_type, self.resource.get_raw());
            gl::BufferSubData(
                self.buffer_type,
                (pos * std::mem::size_of::<T>()) as _,
                (data.len() * std::mem::size_of::<T>()) as _,
                data.as_ptr() as _,
            );
        }

        Ok(())
    }
}
