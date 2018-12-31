use gl::types::*;

use cgmath::Matrix;

pub struct Shader {
    handle: GLuint
}

impl Shader {
    pub fn new(files: Vec<Box<(GLenum, String)>>) -> Result<Shader, String> {
        unsafe {
            let handle = gl::CreateProgram();

            for b in files.into_iter() {
                let (shader_type, file_name) = *b;
                if !vec![gl::VERTEX_SHADER, gl::FRAGMENT_SHADER,
                    gl::COMPUTE_SHADER, gl::GEOMETRY_SHADER].contains(&shader_type) {
                    return Err(format!("Shader type for {} isn\'t supported!", file_name));
                }

                if let Ok(src) = std::fs::read_to_string(file_name.to_owned()) {
                    let c_src = std::ffi::CString::new((&src).as_bytes()).unwrap();

                    let shader_handle = gl::CreateShader(shader_type);

                    gl::ShaderSource(shader_handle, 1, &c_src.as_ptr(), std::ptr::null());
                    gl::CompileShader(shader_handle);

                    let mut status = gl::FALSE as GLint;
                    gl::GetShaderiv(shader_handle, gl::COMPILE_STATUS, &mut status);

                    if status != (gl::TRUE as GLint) {
                        let mut len = 0;
                        gl::GetShaderiv(shader_handle, gl::INFO_LOG_LENGTH, &mut len);
                        
                        let mut buf = Vec::with_capacity(len as usize);
                        buf.set_len((len as usize) - 1);
                        gl::GetShaderInfoLog(shader_handle, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

                        return Err(format!("Couldnt compile {}: {}", file_name,
                                            std::str::from_utf8(&buf).unwrap()));
                    }

                    gl::AttachShader(handle, shader_handle);
                } else {
                    return Err(format!("Couldnt open {}", file_name));
                }
            }

            gl::LinkProgram(handle);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(handle, gl::LINK_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                return Err("Shader failed to link!".to_string());
            }

            Ok(Shader { handle: handle })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    pub fn uniform_mat4x4(&self, name: String, mat: cgmath::Matrix4<f32>) {
        unsafe {
            let location = gl::GetUniformLocation(self.handle,
                            std::ffi::CString::new(name.as_bytes()).unwrap().as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE as GLboolean, mat.as_ptr());
        }
    }

    pub fn uniform_float32(&self, name: String, f: f32) {
        unsafe {
            let location = gl::GetUniformLocation(self.handle,
                            std::ffi::CString::new(name.as_bytes()).unwrap().as_ptr());
            gl::Uniform1f(location, f);
        }
    }
}
