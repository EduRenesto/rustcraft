use gl::types::*;

use stb_image::image;

pub struct Texture {
    handle: GLuint 
}

impl Texture {
    // TODO refactor this
    pub fn from_file(file: String, min_filter: GLint, mag_filter: GLint) -> Result<Texture, String> {
        let data = image::load(file);

        match data {
            image::LoadResult::Error(err) => return Err(err),
            image::LoadResult::ImageU8(img) => {
                let tex = Texture::new();
                tex.bind();

                let internal_format = {
                    if img.depth == 4 {
                        gl::RGBA
                    } else {
                        gl::RGB
                    }
                };

                unsafe {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        internal_format as i32,
                        img.width as GLsizei,
                        img.height as GLsizei,
                        0,
                        internal_format,
                        gl::UNSIGNED_BYTE,
                        img.data.as_ptr() as *const GLvoid
                    ); check_gl!();

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter);
                }

                return Ok(tex);
            },
            image::LoadResult::ImageF32(img) => {
                let tex = Texture::new();
                tex.bind();

                let internal_format = {
                    if img.depth == 4 {
                        gl::RGBA
                    } else {
                        gl::RGB
                    }
                };

                unsafe {
                    gl::TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        internal_format as i32,
                        img.width as GLsizei,
                        img.height as GLsizei,
                        0,
                        internal_format,
                        gl::FLOAT,
                        img.data.as_ptr() as *const GLvoid
                    ); check_gl!();

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter);
                }

                return Ok(tex);
            }
        }
    } 

    pub fn new() -> Texture {
        unsafe {
            let mut handle = 0 as GLuint;
            gl::GenTextures(1, &mut handle);

            Texture { handle: handle }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
        }
    }
}
