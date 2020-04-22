extern crate gl;
extern crate rand;

use core::f32::consts::PI;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Euler, Matrix4, Rad, vec3, Vector3, Vector4};

use crate::drawable::Drawable;
use crate::shader::Shader;

use self::gl::types::*;
use self::rand::Rng;

type VBO = u32;
type VAO = u32;
type EBO = u32;

const SNOW_X_MIN: f32 = -10.;
const SNOW_X_MAX: f32 = 10.;
const SNOW_Y_MIN: f32 = -5.;
const SNOW_Y_MAX: f32 = 10.;
const SNOW_Z_MIN: f32 = -10.;
const SNOW_Z_MAX: f32 = 10.;
const MAX_FLAKES: u32 = 1000;

struct Instance {
    position: Vector3<f32>,
    rotation: Vector3<Rad<f32>>,
}

pub struct Snow {
    shader: Shader,
    vao: VAO,
    instances_vbo: VBO,
    indices: Vec<u32>,
    instances: Vec<Instance>,
}

impl Snow {
    pub fn new() -> Self {
        let shader = Shader::new("src/xmas_tree/shaders/snow.vert", "src/xmas_tree/shaders/static.frag");
        let (vertices, indices) = Snow::gen_objects();
        let instances = Snow::gen_instances();
        let instances_vbo = Self::create_instances_vbo();
        let mut snow = Self { shader, vao: 0, instances_vbo, indices, instances };
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
            // model matrix with rotation and translation
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo);
            let mat4_size = mem::size_of::<Matrix4<f32>>() as i32;
            let vec4_size = mem::size_of::<Vector4<f32>>() as i32;
            // I need to do the calls below 4 times, because size can be at most 4, but I'm sending a matrix of size 16
            gl::VertexAttribPointer(3, 4, gl::FLOAT, gl::FALSE, mat4_size, ptr::null());
            gl::VertexAttribPointer(4, 4, gl::FLOAT, gl::FALSE, mat4_size, vec4_size as *const c_void);
            gl::VertexAttribPointer(5, 4, gl::FLOAT, gl::FALSE, mat4_size, (2 * vec4_size) as *const c_void);
            gl::VertexAttribPointer(6, 4, gl::FLOAT, gl::FALSE, mat4_size, (3 * vec4_size) as *const c_void);
            gl::EnableVertexAttribArray(3);
            gl::EnableVertexAttribArray(4);
            gl::EnableVertexAttribArray(5);
            gl::EnableVertexAttribArray(6);
            gl::VertexAttribDivisor(3, 1);    // every iteration
            gl::VertexAttribDivisor(4, 1);    // every iteration
            gl::VertexAttribDivisor(5, 1);    // every iteration
            gl::VertexAttribDivisor(6, 1);    // every iteration

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

    fn gen_instances() -> Vec<Instance> {
        let mut instances: Vec<Instance> = Vec::with_capacity(MAX_FLAKES as usize);
        let mut rng = rand::thread_rng();
        for _i in 0..MAX_FLAKES {
            let x_position = rng.gen::<f32>() * (SNOW_X_MAX - SNOW_X_MIN) + SNOW_X_MIN;
            let y_position = rng.gen::<f32>() * (SNOW_Y_MAX - SNOW_Y_MIN) + SNOW_Y_MIN;
            let z_position = rng.gen::<f32>() * (SNOW_Z_MAX - SNOW_Z_MIN) + SNOW_Z_MIN;
            let x_rotation = Rad(rng.gen::<f32>() * 2. * PI);
            let y_rotation = Rad(rng.gen::<f32>() * 2. * PI);
            let z_rotation = Rad(rng.gen::<f32>() * 2. * PI);
            let position = vec3(x_position, y_position, z_position);
            let rotation = vec3(x_rotation, y_rotation, z_rotation);
            instances.push(Instance { position, rotation });
        }
        instances
    }

    fn fill_instances_vbo(&self) {
        let mut buffer: Vec<Matrix4<f32>> = Vec::with_capacity(MAX_FLAKES as usize);
        for i in 0..MAX_FLAKES as usize {
            let instance = &self.instances[i];
            let rotation = Matrix4::from(Euler { x: instance.rotation.x, y: instance.rotation.y, z: instance.rotation.z });
            let translation = Matrix4::from_translation(instance.position);
            let model = translation * rotation;
            buffer.push(model);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            let matrix_size = mem::size_of::<Matrix4<f32>>();
            gl::BufferData(gl::ARRAY_BUFFER,
                           (MAX_FLAKES as usize * matrix_size) as GLsizeiptr,
                           buffer.as_ptr() as *const c_void,
                           gl::STATIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
        }
    }

    fn create_instances_vbo() -> VBO {
        unsafe {
            let mut instances_vbo = 0 as VBO;
            gl::GenBuffers(1, &mut instances_vbo); // create buffer for my data
            gl::BindBuffer(gl::ARRAY_BUFFER, instances_vbo); // ARRAY_BUFFER now "points" to my buffer
            let matrix_size = mem::size_of::<Matrix4<f32>>();
            gl::BufferData(gl::ARRAY_BUFFER,
                           (MAX_FLAKES as usize * matrix_size) as GLsizeiptr,
                           ptr::null(),
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
            self.fill_instances_vbo();
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), MAX_FLAKES as i32);
            gl::BindVertexArray(0);
        }
    }
}
