use gl;
use gl::types::*;

use crate::mesh::Mesh;

pub struct VertexBuffer {
    handle: GLuint,
    total_vertices: usize
}

impl VertexBuffer {
    pub fn from_mesh(mesh: Mesh) -> VertexBuffer {
        let mut handle = 0 as GLuint;
        let mut total_vertices = 0 as usize;

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
            gl::BindVertexArray(handle);

            match mesh.positions {
                Some(positions) => {
                    let mut pos_handle = 0 as GLuint;
                    gl::GenBuffers(1, &mut pos_handle);
                    gl::BindBuffer(gl::ARRAY_BUFFER, pos_handle);
                    gl::BufferData(gl::ARRAY_BUFFER, (positions.len() * 3 * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                        positions.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
                    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());
                    gl::EnableVertexAttribArray(0);

                    if total_vertices == 0 {
                        total_vertices = positions.len();
                    }
                }, 
                None => {}
            }

            match mesh.normals {
                Some(normals) => {
                    let mut norm_handle = 0 as GLuint;
                    gl::GenBuffers(1, &mut norm_handle);
                    gl::BindBuffer(gl::ARRAY_BUFFER, norm_handle);
                    gl::BufferData(gl::ARRAY_BUFFER, (normals.len() * 3 * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                        normals.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
                    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());
                    gl::EnableVertexAttribArray(1);

                    if total_vertices == 0 {
                        total_vertices = normals.len();
                    }
                }, 
                None => {}
            }

            match mesh.tex_coords {
                Some(tex_coords) => {
                    let mut uvs_handle = 0 as GLuint;
                    gl::GenBuffers(1, &mut uvs_handle);
                    gl::BindBuffer(gl::ARRAY_BUFFER, uvs_handle);
                    gl::BufferData(gl::ARRAY_BUFFER, (tex_coords.len() * 2 * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                        tex_coords.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
                    gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());
                    gl::EnableVertexAttribArray(2);

                    if total_vertices == 0 {
                        total_vertices = tex_coords.len();
                    }
                }, 
                None => {}
            }

            match mesh.occlusion {
                Some(occlusion) => {
                    let mut occlusion_handle = 0 as GLuint;
                    gl::GenBuffers(1, &mut occlusion_handle);
                    gl::BindBuffer(gl::ARRAY_BUFFER, occlusion_handle);
                    gl::BufferData(gl::ARRAY_BUFFER, (occlusion.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                        occlusion.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
                    gl::VertexAttribPointer(3, 1, gl::FLOAT, gl::FALSE as GLboolean, 0, std::ptr::null());
                    gl::EnableVertexAttribArray(3);
                },
                None => {}
            }
        }

        VertexBuffer { handle: handle, total_vertices: total_vertices }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray(self.handle); check_gl!();
            gl::DrawArrays(gl::TRIANGLES, 0, self.total_vertices as i32); check_gl!();
        }
    }
}
