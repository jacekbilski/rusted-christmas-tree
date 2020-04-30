extern crate gl;
extern crate rand;

use core::f32::consts::PI;

use cgmath::{Euler, Matrix4, Point3, Rad, vec3, Vector3};
use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::rngs::SmallRng;

use crate::material::{Material, MaterialId, Materials};
use crate::model::{Instance, Model};
use crate::shader::Shader;
use crate::xmas_tree::mesh::{Mesh, Vertex};

const SNOW_X_MIN: f32 = -10.;
const SNOW_X_MAX: f32 = 10.;
const SNOW_Y_MIN: f32 = -5.;
const SNOW_Y_MAX: f32 = 10.;
const SNOW_Z_MIN: f32 = -10.;
const SNOW_Z_MAX: f32 = 10.;

const SNOWFLAKE_FALL_VELOCITY: f32 = 0.01;
const SNOWFLAKE_MAX_RANDOM_OFFSET: f32 = 0.01;
const SNOWFLAKE_MAX_RANDOM_ROTATION: f32 = PI / 180. * 10.;
const MAX_SNOWFLAKES: usize = 5_000;

struct Snowflake {
    position: Vector3<f32>,
    rotation: Vector3<Rad<f32>>,
}

pub struct Snow {
    mesh: Mesh,
    snowflakes: Vec<Snowflake>,
    material_id: MaterialId,
}

impl Snow {
    pub fn new(materials: &mut Materials) -> Self {
        let ambient: Vector3<f32> = vec3(1., 1., 1.);
        let diffuse: Vector3<f32> = vec3(0.623960, 0.686685, 0.693872);
        let specular: Vector3<f32> = vec3(0.5, 0.5, 0.5);
        let shininess: f32 = 225.;
        let material = Material { ambient, diffuse, specular, shininess };
        let material_id = materials.add(material);

        let (vertices, indices) = Snow::gen_snowflake_mesh();
        let mesh = Mesh::new(vertices, indices, MAX_SNOWFLAKES);

        let snowflakes = Snow::gen_snowflakes();
        let snow = Self { mesh, snowflakes, material_id };
        let instances = snow.gen_instances();
        snow.mesh.fill_instances_vbo(&instances);
        snow
    }

    fn gen_snowflake_mesh() -> (Vec<Vertex>, Vec<u32>) {
        let radius: f32 = 0.05;
        let normal: Vector3<f32> = vec3(1., 0., 0.);
        let mut vertices: Vec<Vertex> = vec![];

        let angle_diff = PI / 3 as f32;

        for i in 0..6 {
            let angle = i as f32 * angle_diff;
            // upper side
            vertices.push(Vertex { position: Point3::new(0., radius * angle.cos(), radius * angle.sin()), normal });
            // bottom side
            vertices.push(Vertex { position: Point3::new(-0., -radius * angle.cos(), -radius * angle.sin()), normal: -normal });
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

    fn gen_snowflakes() -> Vec<Snowflake> {
        let mut snowflakes: Vec<Snowflake> = Vec::with_capacity(MAX_SNOWFLAKES as usize);
        let x_range = Uniform::new(SNOW_X_MIN, SNOW_X_MAX);
        let y_range = Uniform::new(SNOW_Y_MIN, SNOW_Y_MAX);
        let z_range = Uniform::new(SNOW_Z_MIN, SNOW_Z_MAX);
        let angle_range = Uniform::new(0., 2. * PI);
        let mut rng = SmallRng::from_entropy();
        for _i in 0..MAX_SNOWFLAKES {
            let x_position = rng.sample(x_range);
            let y_position = rng.sample(y_range);
            let z_position = rng.sample(z_range);
            let x_rotation = Rad(rng.sample(angle_range));
            let y_rotation = Rad(rng.sample(angle_range));
            let z_rotation = Rad(rng.sample(angle_range));
            let position = vec3(x_position, y_position, z_position);
            let rotation = vec3(x_rotation, y_rotation, z_rotation);
            snowflakes.push(Snowflake { position, rotation });
        }
        snowflakes
    }

    fn move_snowflakes(&mut self) {
        let mut rng = SmallRng::from_entropy();
        let pos_offset_range = Uniform::new(-SNOWFLAKE_MAX_RANDOM_OFFSET as f32, SNOWFLAKE_MAX_RANDOM_OFFSET);
        let rot_angle_range = Uniform::new(-SNOWFLAKE_MAX_RANDOM_ROTATION, SNOWFLAKE_MAX_RANDOM_ROTATION);
        for i in 0..MAX_SNOWFLAKES as usize {
            let mut snowflake = &mut self.snowflakes[i];
            let new_x_pos = snowflake.position.x + rng.sample(pos_offset_range);
            let mut new_y_pos = snowflake.position.y + rng.sample(pos_offset_range) - SNOWFLAKE_FALL_VELOCITY;
            if new_y_pos < SNOW_Y_MIN {
                new_y_pos = SNOW_Y_MAX;
            }
            let new_z_pos = snowflake.position.z + rng.sample(pos_offset_range);
            snowflake.position = vec3(new_x_pos, new_y_pos, new_z_pos);

            let new_x_rot = snowflake.rotation.x + Rad(rng.sample(rot_angle_range));
            let new_y_rot = snowflake.rotation.y + Rad(rng.sample(rot_angle_range));
            let new_z_rot = snowflake.rotation.z + Rad(rng.sample(rot_angle_range));
            snowflake.rotation = vec3(new_x_rot, new_y_rot, new_z_rot);
        }
    }

    fn gen_instances(&self) -> Vec<Instance> {
        let mut instances: Vec<Instance> = Vec::with_capacity(MAX_SNOWFLAKES);
        for i in 0..MAX_SNOWFLAKES {
            let snowflake = &self.snowflakes[i];
            let rotation = Matrix4::from(Euler { x: snowflake.rotation.x, y: snowflake.rotation.y, z: snowflake.rotation.z });
            let translation = Matrix4::from_translation(snowflake.position);
            let model = translation * rotation;
            instances.push(Instance { model, material_id: self.material_id });
        }
        instances
    }
}

impl Model for Snow {
    fn next_frame(&mut self) {
        self.move_snowflakes();
        let instances = self.gen_instances();
        self.mesh.fill_instances_vbo(&instances);
    }

    fn draw(&mut self, shader: &Shader) {
        self.mesh.draw_instances(shader, MAX_SNOWFLAKES);
    }
}
