extern crate gl;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Point3, Vector3};

use crate::drawable::Drawable;
use crate::material::Material;
use crate::shader::Shader;

use self::gl::types::*;

type VBO = u32;
type VAO = u32;
type EBO = u32;

#[repr(C)]  // to make sure memory representation is like in the code
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    fn size() -> usize {
        let float_size = mem::size_of::<GLfloat>();
        2 * 3 * float_size
    }
}

pub struct Mesh {
    vao: VAO,
    indices: Vec<u32>,
    material: Material,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, material: Material) -> Self {
        let within_vao = || {
            Self::create_vbo(&vertices);
            Self::create_ebo(&indices);
        };
        let vao = Self::create_vao(within_vao);

        Self { vao, indices, material }
    }

    fn create_vao(within_vao_context: impl Fn() -> ()) -> VAO {
        unsafe {
            let mut vao = 0 as VAO;
            gl::GenVertexArrays(1, &mut vao); // create VAO
            gl::BindVertexArray(vao); // ...and bind it

            within_vao_context();

            let stride = Vertex::size() as GLsizei;
            // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
            // this call also connects my VBO to this attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0); // enable the attribute for position

            // second three floats are for normal vector
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1); // enable the attribute for colour

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind my VBO
            // do NOT unbind EBO, VAO would remember that
            gl::BindVertexArray(0); // unbind my VAO
            vao
        }
    }

    fn create_vbo(vertices: &Vec<Vertex>) {
        unsafe {
            let mut vbo = 0 as VBO;
            gl::GenBuffers(1, &mut vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * Vertex::size()) as GLsizeiptr,
                           &vertices[0] as *const Vertex as *const c_void,
                           gl::STATIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_ebo(indices: &[u32]) {
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

impl Drawable for Mesh {
    fn draw(&mut self, shader: &Shader) {
        unsafe {
            gl::UseProgram(shader.id);
            gl::BindVertexArray(self.vao);
            shader.set_vector3("material.ambient", self.material.ambient);
            shader.set_vector3("material.diffuse", self.material.diffuse);
            shader.set_vector3("material.specular", self.material.specular);
            shader.set_float("material.shininess", self.material.shininess);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
