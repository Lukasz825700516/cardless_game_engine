use std::ffi::c_void;

pub enum BufferType {
    VERTEX,
    ELEMENT,
}

pub struct Buffer<T>
where T: Sized {
    pub handler: u32,
    pub data: Vec<T>,
    buffer_type: u32,
    t_size: usize,
}

impl<T> Buffer<T>
where T: Sized {
    pub fn new(buffer_type: BufferType, data: Vec<T>, size_of: usize) -> Self {
        let mut handler = 0;
        let buffer_type = match buffer_type {
            BufferType::VERTEX => gl::ARRAY_BUFFER,
            BufferType::ELEMENT => gl::ELEMENT_ARRAY_BUFFER,
        };

        let bytes_len = data.len() * size_of;
        let bytes_ptr = data.as_ptr() as *const c_void;

        unsafe { gl::GenBuffers(1, &mut handler); }
        unsafe { gl::BindBuffer(buffer_type, handler); }
        unsafe { gl::BufferData(buffer_type, bytes_len as isize, bytes_ptr, gl::STATIC_DRAW); }

        Self { handler, data, t_size: size_of, buffer_type }
    }

    pub fn flush(&mut self) {
        let bytes_len = self.data.len() * self.t_size;
        let bytes_ptr = self.data.as_ptr() as *const c_void;


        unsafe { gl::BindBuffer(self.buffer_type, self.handler); }
        unsafe { gl::BufferData(self.buffer_type, bytes_len as isize, bytes_ptr, gl::DYNAMIC_DRAW); }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.handler); }
    }
}

