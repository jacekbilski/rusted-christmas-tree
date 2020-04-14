extern crate gl;

use core::f32::consts::PI;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Deg, Matrix4, perspective, Point3, vec3};
use cgmath::prelude::*;
use glfw::Window;

use crate::drawable::Drawable;
use crate::shader::Shader;

use self::gl::types::*;

type VBO = u32;
type VAO = u32;
type EBO = u32;

pub struct Tree {
    shader: Shader,
    vao: VAO,
    indices: Vec<u32>,
}

impl Tree {
    pub fn setup() -> Self {
        let shader = Shader::new();

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let (vertices, indices) = Tree::gen_vertices();

        let within_vao = || {
            Tree::create_vbo(&vertices);
            Tree::create_ebo(&indices);
        };

        let vao = Tree::create_vao(within_vao);

        Tree { shader, vao, indices }
    }

    fn gen_vertices() -> (Vec<f32>, Vec<u32>) {
        let slices = 40 as u32;
        let colour: [f32; 3] = [0., 1., 0.];

        let mut vertices: Vec<f32> = Vec::with_capacity(9 * (slices + 1) as usize);
        let mut indices: Vec<u32> = Vec::with_capacity(3 * (slices + 1) as usize);

        // lower segment
        Tree::gen_tree_segment(slices, colour, &mut vertices, &mut indices, 4., 0, -3., 5.);

        // middle segment
        Tree::gen_tree_segment(slices, colour, &mut vertices, &mut indices, 3., 1, 0., 3.);

        // upper segment
        Tree::gen_tree_segment(slices, colour, &mut vertices, &mut indices, 2., 2, 2., 2.);
        (vertices, indices)
    }

    fn gen_tree_segment(slices: u32, colour: [f32; 3], vertices: &mut Vec<f32>, indices: &mut Vec<u32>, bottom_radius: f32, segment: u32, segment_bottom: f32, segment_height: f32) {
        let angle_diff = PI * 2. / slices as f32;
        let indices_offset = 2 * segment * slices;
        let upper_radius: f32 = 0.001;
        for i in 0..slices {
            let bottom_angle = angle_diff * i as f32;
            let upper_angle = angle_diff * (i as f32 - 0.5);

            // bottom edge
            vertices.extend([bottom_radius * bottom_angle.sin(), segment_bottom, bottom_radius * bottom_angle.cos()].iter());
            vertices.extend(colour.iter());
            vertices.extend([bottom_angle.sin(), 0.7, bottom_angle.cos()].iter()); // TODO - actually calculate this

            // upper edge
            vertices.extend([upper_radius * upper_angle.sin(), segment_bottom + segment_height, upper_radius * upper_angle.cos()].iter());
            vertices.extend(colour.iter());
            vertices.extend([upper_angle.sin(), segment_bottom + segment_height + 0.7, upper_angle.cos()].iter());  // TODO - actually calculate this

            if i != slices - 1 {
                indices.extend([indices_offset + 2 * i, indices_offset + 2 * i + 1, indices_offset + 2 * i + 2].iter());
                indices.extend([indices_offset + 2 * i + 1, indices_offset + 2 * i + 2, indices_offset + 2 * i + 3].iter());
            }
        }
        indices.extend([indices_offset + 2 * (slices - 1), indices_offset + 2 * (slices - 1) + 1, indices_offset].iter());
        indices.extend([indices_offset + 2 * (slices - 1) + 1, indices_offset, indices_offset + 1].iter());
    }

    fn create_vao(within_vao_context: impl Fn() -> ()) -> VAO {
        unsafe {
            let mut vao = 0 as VAO;
            gl::GenVertexArrays(1, &mut vao); // create VAO
            gl::BindVertexArray(vao); // ...and bind it

            within_vao_context();

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
}

impl Drawable for Tree {
    fn draw(&self, window: &mut Window) {
        unsafe {
            let camera_position = Point3::new(15., 12., 12.);
            gl::UseProgram(self.shader.id);
            self.shader.set_vec3("lightColour", &vec3(1., 1., 1.));
            // self.shader.set_vec3("lightPosition", &vec3(10., 100., 10.));
            self.shader.set_vec3("lightPosition", &vec3(5., 6., 2.));
            self.shader.set_vec3("cameraPosition", &camera_position.to_vec());

            self.shader.set_mat4("model", &Matrix4::identity() as &Matrix4<f32>);
            let view: Matrix4<f32> = Matrix4::look_at(camera_position, Point3::new(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
            self.shader.set_mat4("view", &view);
            let (width, height) = window.get_size();
            let projection = perspective(Deg(45.0), width as f32 / height as f32, 0.1, 100.0);
            self.shader.set_mat4("projection", &projection);

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}
