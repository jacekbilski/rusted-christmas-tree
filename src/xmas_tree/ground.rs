extern crate gl;

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Deg, Matrix4, perspective, vec3};
use cgmath::prelude::*;

use crate::drawable::Drawable;
use crate::shader::Shader;

use self::gl::types::*;

type VBO = u32;
type VAO = u32;
type EBO = u32;

pub struct Ground {
    shader: Shader,
    vao: VAO,
}

impl Ground {
    pub fn setup() -> Ground {
        let shader = Shader::new();

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 24] = [
            -0.8, -0.8, 0.0, 1.0, 1.0, 1.0, // bottom left
            -0.8, 0.8, 0.0, 1.0, 1.0, 1.0, // top left
            0.8, 0.8, 0.0, 1.0, 1.0, 1.0, // top right
            0.8, -0.8, 0.0, 1.0, 1.0, 1.0, // bottom right
        ];
        let indices: [u32; 6] = [
            0, 1, 2,
            2, 3, 0,
        ];

        let within_vao = || {
            Ground::create_vbo(&vertices);
            Ground::create_ebo(&indices);
        };

        let vao = Ground::create_vao(within_vao);

        Ground{shader, vao}
    }

    fn create_vao(within_vao_context: impl Fn() -> ()) -> VAO {
        unsafe {
            let mut vao = 0 as VAO;
            gl::GenVertexArrays(1, &mut vao); // create VAO
            gl::BindVertexArray(vao); // ...and bind it

            within_vao_context();

            let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
            // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
            // this call also connects my VBO to this attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0); // enable the attribute for position
            // second three floats are for colour, last param is used to point to values within single vertex
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1); // enable the attribute for colour

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind my VBO
            // do NOT unbind EBO, VAO would remember that
            gl::BindVertexArray(0); // unbind my VAO
            vao
        }
    }

    fn create_vbo(vertices: &[f32; 24]) {
        unsafe {
            let mut vbo = 0 as VBO;
            gl::GenBuffers(1, &mut vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_ebo(indices: &[u32; 6]) {
        unsafe {
            let mut ebo = 0 as EBO;
            gl::GenBuffers(1, &mut ebo); // create buffer for indices (elements)
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); // ELEMENT_ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                           &indices[0] as *const u32 as *const c_void,
                           gl::STATIC_DRAW); // actually fill ELEMENT_ARRAY_BUFFER with data
        }
    }
}

impl Drawable for Ground {
    fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader.id);

            self.set_uniform_matrix("model", Matrix4::identity());
            let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -3.));
            self.set_uniform_matrix("view", view);
            let projection = perspective(Deg(45.0), 1920 as f32 / 1080 as f32, 0.1, 100.0);
            self.set_uniform_matrix("projection", projection);

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl Ground {
    fn set_uniform_matrix(&self, name: &str, matrix: Matrix4<f32>) {
        unsafe {
            let c_name= CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.shader.id, c_name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }
}
