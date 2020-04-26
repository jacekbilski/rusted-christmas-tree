extern crate gl;
extern crate rand;

use core::f32::consts::PI;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{Euler, Matrix4, Rad, vec3, Vector3, Vector4};
use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::rngs::SmallRng;

use crate::drawable::Drawable;
use crate::material::Material;
use crate::shader::Shader;

use self::gl::types::*;

type VBO = u32;
type VAO = u32;
type EBO = u32;

const SNOW_X_MIN: f32 = -10.;
const SNOW_X_MAX: f32 = 10.;
const SNOW_Y_MIN: f32 = -5.;
const SNOW_Y_MAX: f32 = 10.;
const SNOW_Z_MIN: f32 = -10.;
const SNOW_Z_MAX: f32 = 10.;

const SNOWFLAKE_FALL_VELOCITY: f32 = 0.01;
const SNOWFLAKE_MAX_RANDOM_OFFSET: f32 = 0.01;
const SNOWFLAKE_MAX_RANDOM_ROTATION: f32 = PI / 180. * 10.;
const MAX_FLAKES: u32 = 5_000;

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
    material: Material,
}

impl Snow {
    pub fn new() -> Self {
        let shader = Shader::new("src/xmas_tree/shaders/snow.vert", "src/xmas_tree/shaders/static.frag");
        let (vertices, indices) = Snow::gen_objects();
        let instances = Snow::gen_instances();
        let instances_vbo = Self::create_instances_vbo();

        let ambient: Vector3<f32> = vec3(1., 1., 1.);
        let diffuse: Vector3<f32> = vec3(0.623960, 0.686685, 0.693872);
        let specular: Vector3<f32> = vec3(0.5, 0.5, 0.5);
        let shininess: f32 = 225.;
        let material = Material { ambient, diffuse, specular, shininess };

        let mut snow = Self { shader, vao: 0, instances_vbo, indices, instances, material };
        snow.vao = snow.create_vao(&vertices);
        snow
    }

    fn gen_objects() -> (Vec<f32>, Vec<u32>) {
        let radius: f32 = 0.05;
        let normal: [f32; 3] = [1., 0., 0.];
        let neg_normal: [f32; 3] = [-1., 0., 0.];
        let mut vertices: Vec<f32> = vec![];

        let angle_diff = PI / 3 as f32;

        for i in 0..6 {
            let angle = i as f32 * angle_diff;
            // upper side
            vertices.extend([0., radius * angle.cos(), radius * angle.sin()].iter());
            vertices.extend(normal.iter());
            // bottom side
            vertices.extend([-0., -radius * angle.cos(), -radius * angle.sin()].iter());
            vertices.extend(neg_normal.iter());
        }
        let indices: Vec<u32> = vec![
            // upper side
            8, 4, 0,
            10, 6, 2,
            // bottom side
            1, 5, 9,
            3, 7, 11,
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

            let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
            // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
            // this call also connects my VBO to this attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0); // enable the attribute for position

            // second three floats are for normal vector
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1); // enable the attribute for colour

            // enter instancing, using completely different VBO
            // model matrix with rotation and translation
            gl::BindBuffer(gl::ARRAY_BUFFER, self.instances_vbo);
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
        let x_range = Uniform::new(SNOW_X_MIN, SNOW_X_MAX);
        let y_range = Uniform::new(SNOW_Y_MIN, SNOW_Y_MAX);
        let z_range = Uniform::new(SNOW_Z_MIN, SNOW_Z_MAX);
        let angle_range = Uniform::new(0., 2. * PI);
        let mut rng = SmallRng::from_entropy();
        for _i in 0..MAX_FLAKES {
            let x_position = rng.sample(x_range);
            let y_position = rng.sample(y_range);
            let z_position = rng.sample(z_range);
            let x_rotation = Rad(rng.sample(angle_range));
            let y_rotation = Rad(rng.sample(angle_range));
            let z_rotation = Rad(rng.sample(angle_range));
            let position = vec3(x_position, y_position, z_position);
            let rotation = vec3(x_rotation, y_rotation, z_rotation);
            instances.push(Instance { position, rotation });
        }
        instances
    }

    fn move_snowflakes(&mut self) {
        let mut rng = SmallRng::from_entropy();
        let pos_offset_range = Uniform::new(-SNOWFLAKE_MAX_RANDOM_OFFSET as f32, SNOWFLAKE_MAX_RANDOM_OFFSET);
        let rot_angle_range = Uniform::new(-SNOWFLAKE_MAX_RANDOM_ROTATION, SNOWFLAKE_MAX_RANDOM_ROTATION);
        for i in 0..MAX_FLAKES as usize {
            let mut instance = &mut self.instances[i];
            let new_x_pos = instance.position.x + rng.sample(pos_offset_range);
            let mut new_y_pos = instance.position.y + rng.sample(pos_offset_range) - SNOWFLAKE_FALL_VELOCITY;
            if new_y_pos < SNOW_Y_MIN {
                new_y_pos = SNOW_Y_MAX;
            }
            let new_z_pos = instance.position.z + rng.sample(pos_offset_range);
            instance.position = vec3(new_x_pos, new_y_pos, new_z_pos);

            let new_x_rot = instance.rotation.x + Rad(rng.sample(rot_angle_range));
            let new_y_rot = instance.rotation.y + Rad(rng.sample(rot_angle_range));
            let new_z_rot = instance.rotation.z + Rad(rng.sample(rot_angle_range));
            instance.rotation = vec3(new_x_rot, new_y_rot, new_z_rot);
        }
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
                           gl::DYNAMIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
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
                           ptr::null(), // don't fill, only reserve space
                           gl::DYNAMIC_DRAW);
            instances_vbo
        }
    }

    fn load_material(&self) {
        self.shader.set_vector3("material.ambient", self.material.ambient);
        self.shader.set_vector3("material.diffuse", self.material.diffuse);
        self.shader.set_vector3("material.specular", self.material.specular);
        self.shader.set_float("material.shininess", self.material.shininess);
    }
}

impl Drawable for Snow {
    fn draw(&mut self) {
        self.move_snowflakes();
        self.fill_instances_vbo();
        unsafe {
            gl::UseProgram(self.shader.id);
            gl::BindVertexArray(self.vao);
            self.load_material();
            gl::DrawElementsInstanced(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null(), MAX_FLAKES as i32);
            gl::BindVertexArray(0);
        }
    }
}
