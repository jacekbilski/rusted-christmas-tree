extern crate gl;

use core::f32::consts::PI;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use crate::drawable::Drawable;
use crate::shader::Shader;

use self::gl::types::*;

type VBO = u32;
type VAO = u32;
type EBO = u32;

pub struct Snow {
    shader: Shader,
    vao: VAO,
    indices: Vec<u32>,
    instances_vbo: VBO,
}

impl Snow {
    pub fn new() -> Self {
        let shader = Shader::new("src/xmas_tree/shaders/snow.vert", "src/xmas_tree/shaders/static.frag");
        let (vertices, indices) = Snow::gen_objects();
        let instances_vbo = Self::create_instances_vbo();
        let mut snow = Self { shader, vao: 0, indices, instances_vbo };
        snow.vao = snow.create_vao(&vertices);
        snow
    }

    fn gen_objects() -> (Vec<f32>, Vec<u32>) {
        let radius: f32 = 0.07;
        let colour: [f32; 3] = [1., 1., 1.];
        let normal: [f32; 3] = [1., 0., 0.];
        let mut vertices: Vec<f32> = vec![];

        let angle_diff = PI / 3 as f32;

        for i in 0..6 {
            let angle = i as f32 * angle_diff;
            vertices.extend([0., radius * angle.cos(), radius * angle.sin()].iter());
            vertices.extend(colour.iter());
            vertices.extend(normal.iter());
        }
        let indices: Vec<u32> = vec![
            4, 2, 0,
            5, 3, 1,
        ];

        (vertices, indices)
    }

    fn create_vao(&self, vertices: &[f32]) -> VAO {
        unsafe {
            let mut vao = 0 as VAO;
            gl::GenVertexArrays(1, &mut vao); // create VAO
            gl::BindVertexArray(vao); // ...and bind it

            Self::create_vbo(vertices);
            Self::create_ebo(&self.indices);

            let stride = 9 * mem::size_of::<GLfloat>() as GLsizei;
            // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
            // this call also connects my VBO to this attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0); // enable the attribute for position

            // second three floats are for colour, last param is used to point to values within single vertex
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1); // enable the attribute for colour

            // third three floats are for normal vector used for lightning calculations
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(2); // enable the attribute for normal vector

            // enter instancing, using completely different VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo);
            gl::VertexAttribPointer(3, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribDivisor(3, 1);    // every iteration

            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind my VBO
            // do NOT unbind EBO, VAO would remember that
            gl::BindVertexArray(0); // unbind my VAO
            vao
        }
    }

    fn create_vbo(vertices: &[f32]) {
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

    fn create_instances_vbo() -> VBO {
        unsafe {
            let instances: [f32; 9] = [
                0., 0., 0.,
                0., 1., 0.,
                0., 0., 1.,
            ];

            let mut instances_vbo = 0 as VBO;
            gl::GenBuffers(1, &mut instances_vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            gl::BufferData(gl::ARRAY_BUFFER,
                           (instances.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &instances[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data

            instances_vbo
        }
    }
}

impl Drawable for Snow {
    fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), 3);
            gl::BindVertexArray(0);
        }
    }
}
