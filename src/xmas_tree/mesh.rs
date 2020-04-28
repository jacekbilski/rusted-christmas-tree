extern crate gl;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Matrix4, Point3, SquareMatrix, Vector3, Vector4};

use crate::material::Material;
use crate::model::Model;
use crate::shader::Shader;

use self::gl::types::*;

type VBO = u32;
type VAO = u32;
type EBO = u32;

#[repr(C)]  // to make sure memory representation is like in the code
#[derive(Debug)]
pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    pub fn size() -> usize {
        let float_size = mem::size_of::<GLfloat>();
        2 * 3 * float_size
    }
}

pub struct Mesh {
    indices: Vec<u32>,
    material: Material,
    max_instances: u32,
    vao: VAO,
    instances_vbo: VBO,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, material: Material, max_instances: u32) -> Self {
        let instances_vbo = Self::create_instances_vbo(max_instances);
        let vao = Self::create_vao(&vertices, &indices, instances_vbo);
        let mesh = Self { indices, material, max_instances, vao, instances_vbo };
        mesh.fill_instances_vbo(&vec![Matrix4::identity()]);
        mesh
    }

    fn create_vao(vertices: &Vec<Vertex>, indices: &Vec<u32>, instances_vbo: u32) -> VAO {
        unsafe {
            let mut vao = 0 as VAO;
            gl::GenVertexArrays(1, &mut vao); // create VAO
            gl::BindVertexArray(vao); // ...and bind it

            Self::create_vbo(vertices);
            Self::create_ebo(indices);

            let stride = Vertex::size() as GLsizei;
            // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
            // this call also connects my VBO to this attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0); // enable the attribute for position

            // second three floats are for normal vector
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1); // enable the attribute for colour

            // enter instancing, using completely different VBO
            // model matrix with rotation and translation
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo);
            let mat4_size = mem::size_of::<Matrix4<f32>>() as i32;
            let vec4_size = mem::size_of::<Vector4<f32>>() as i32;
            // I need to do the calls below 4 times, because size can be at most 4, but I'm sending a matrix of size 16
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, mat4_size, ptr::null());
            gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, mat4_size, vec4_size as *const c_void);
            gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, mat4_size, (2 * vec4_size) as *const c_void);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, mat4_size, (3 * vec4_size) as *const c_void);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);
            gl::EnableVertexAttribArray(4);
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribDivisor(2, 1);    // every iteration
            gl::VertexAttribDivisor(3, 1);    // every iteration
            gl::VertexAttribDivisor(4, 1);    // every iteration
            gl::VertexAttribDivisor(5, 1);    // every iteration

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

    fn fill_instances_vbo(&self, models: &Vec<Matrix4<f32>>) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            let matrix_size = mem::size_of::<Matrix4<f32>>();
            gl::BufferData(gl::ARRAY_BUFFER,
                           (self.max_instances as usize * matrix_size) as GLsizeiptr,
                           models.as_ptr() as *const c_void,
                           gl::DYNAMIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_instances_vbo(max_instances: u32) -> VBO {
        unsafe {
            let mut instances_vbo = 0 as VBO;
            gl::GenBuffers(1, &mut instances_vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            let matrix_size = mem::size_of::<Matrix4<f32>>();
            gl::BufferData(gl::ARRAY_BUFFER,
                           (max_instances as usize * matrix_size) as GLsizeiptr,
                           ptr::null(), // don't fill, only reserve space
                           gl::DYNAMIC_DRAW);
            instances_vbo
        }
    }

    pub fn draw_single(&self, shader: &Shader) {
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
