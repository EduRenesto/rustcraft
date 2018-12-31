use gl::types::*;

use crate::texture::Texture;

pub struct RenderTarget {
    handle: GLuint,
    pub color_attachments: Vec<Texture>,
    pub depth_attachment: Option<Texture>
}

impl RenderTarget {
    pub fn new(width: usize, height: usize, num_attachments: usize, depth_texture: bool) -> RenderTarget {
        unsafe {
            let mut handle = 0 as GLuint;
            gl::GenFramebuffers(1, &mut handle);
            gl::BindFramebuffer(gl::FRAMEBUFFER, handle);

            let mut textures = Vec::<Texture>::new();

            let mut draw_buffers = Vec::<GLenum>::new();
            
            for i in 0..num_attachments {
                let tex = Texture::new();
                tex.bind();
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as i32,
                               width as i32, height as i32, 0, gl::RGBA, gl::FLOAT, std::ptr::null());
                gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0 + i as u32, gl::TEXTURE_2D, tex.handle, 0);
                check_gl!();
                
                draw_buffers.push(gl::COLOR_ATTACHMENT0 + i as u32);
                textures.push(tex);
            }

            gl::DrawBuffers(draw_buffers.len() as i32, draw_buffers.as_ptr());

            let depth_attachment = if depth_texture {
                let tex = Texture::new();
                tex.bind();
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32,
                               width as i32, height as i32, 0, gl::RGB16F, gl::FLOAT, std::ptr::null());
                gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT as u32, gl::TEXTURE_2D, tex.handle, 0);
                check_gl!();
                Some(tex)
            } else {
                let mut rb_handle = 0 as GLuint;
                gl::GenRenderbuffers(1, &mut rb_handle);
                gl::BindRenderbuffer(gl::RENDERBUFFER, rb_handle);
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width as i32, height as i32);
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT as u32, gl::RENDERBUFFER, rb_handle);
                check_gl!();
                None
            };

            RenderTarget { 
                handle: handle,
                color_attachments: textures,
                depth_attachment: depth_attachment
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.handle);
        }
    }

    pub fn reset() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}
