extern crate gl;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Point3, Vector3, Vector4};

use crate::model::Instance;
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
    max_instances: usize,
    vao: VAO,
    instances_vbo: VBO,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, max_instances: usize) -> Self {
        let instances_vbo = Self::create_instances_vbo(max_instances);
        let vao = Self::create_vao(&vertices, &indices, instances_vbo);
        let mesh = Self { indices, max_instances, vao, instances_vbo };
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
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo);
            let vec4_size = mem::size_of::<Vector4<f32>>() as i32;
            let instances_stride = Instance::size() as GLsizei;
            // println!("Instances stride: {}, Instance.size: {}", instances_stride, Instance::size());

            // model matrix with rotation and translation
            // I need to do the calls below 4 times, because size can be at most 4, but I'm sending a matrix of size 16
            gl::VertexAttribPointer(2, 4, gl::FLOAT, gl::FALSE, instances_stride, ptr::null());
            gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, instances_stride, vec4_size as *const c_void);
            gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, instances_stride, (2 * vec4_size) as *const c_void);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, instances_stride, (3 * vec4_size) as *const c_void);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);
            gl::EnableVertexAttribArray(4);
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribDivisor(2, 1);    // every iteration
            gl::VertexAttribDivisor(3, 1);    // every iteration
            gl::VertexAttribDivisor(4, 1);    // every iteration
            gl::VertexAttribDivisor(5, 1);    // every iteration

            // material_id
            gl::VertexAttribPointer(6, 1, gl::FLOAT, gl::FALSE, instances_stride, (4 * vec4_size) as *const c_void);
            gl::EnableVertexAttribArray(6);
            gl::VertexAttribDivisor(6, 1);    // every iteration

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind instances VBO
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

    pub fn fill_instances_vbo(&self, instances: &Vec<Instance>) {
        // println!("Instance[0]: {:?}", instances[0]);
        // println!("Instance: {:?}", instances);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (self.max_instances * Instance::size()) as GLsizeiptr,
                           instances.as_ptr() as *const c_void,
                           gl::DYNAMIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_instances_vbo(max_instances: usize) -> VBO {
        unsafe {
            let mut instances_vbo = 0 as VBO;
            gl::GenBuffers(1, &mut instances_vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (max_instances * Instance::size()) as GLsizeiptr,
                           ptr::null(), // don't fill, only reserve space
                           gl::DYNAMIC_DRAW);
            instances_vbo
        }
    }

    pub fn draw_single(&self, shader: &Shader) {
        unsafe {
            gl::UseProgram(shader.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_instances(&mut self, shader: &Shader, num: usize) {
        unsafe {
            gl::UseProgram(shader.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), num as i32);
            gl::BindVertexArray(0);
        }
    }
}
