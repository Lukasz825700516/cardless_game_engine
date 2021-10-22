use std::{io::{BufRead, Seek}, ffi::c_void};
use image::GenericImageView;

pub struct Texture {
    pub handler: u32,
}

impl Texture {
    pub fn try_load<T>(data: T) -> Option<Self>
    where T: BufRead + Seek {
        let image = image::load(data, image::ImageFormat::Png).unwrap().flipv();
        let mut handler = 0;
        unsafe { gl::GenTextures(1, &mut handler); }
        unsafe { gl::BindTexture(gl::TEXTURE_2D, handler); }
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); }
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); }
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32); }
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); }

        if let Some(colors) = image.as_rgba8() {
            unsafe { gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width() as i32, image.height() as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, colors.as_ptr() as *const c_void); }
        }
        if let Some(colors) = image.as_rgb8() {
            unsafe { gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, image.width() as i32, image.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, colors.as_ptr() as *const c_void); }
        }

        unsafe { gl::GenerateMipmap(gl::TEXTURE_2D); }

        Some(Self {handler})
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.handler); }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.handler); }
    }
}

