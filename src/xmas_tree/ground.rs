use cgmath::{Point3, vec3, Vector3};

use crate::drawable::Drawable;
use crate::material::Material;
use crate::shader::Shader;
use crate::xmas_tree::mesh::{Mesh, Vertex};

pub struct Ground {
    mesh: Mesh,
}

impl Ground {
    pub fn new() -> Self {
        let vertices: Vec<Vertex> = vec![
            Vertex { position: Point3::new(-10., -5., -10.), normal: vec3(0., 1., 0.) },   // far
            Vertex { position: Point3::new(-10., -5., 10.), normal: vec3(0., 1., 0.) }, // left
            Vertex { position: Point3::new(10., -5., -10.), normal: vec3(0., 1., 0.) }, // right
            Vertex { position: Point3::new(10., -5., 10.), normal: vec3(0., 1., 0.) }, // near
        ];

        let indices: Vec<u32> = vec![
            0, 2, 1,
            1, 2, 3,
        ];

        let ambient: Vector3<f32> = vec3(1., 1., 1.);
        let diffuse: Vector3<f32> = vec3(0.623960, 0.686685, 0.693872);
        let specular: Vector3<f32> = vec3(0.5, 0.5, 0.5);
        let shininess: f32 = 225.;
        let material = Material { ambient, diffuse, specular, shininess };

        let mesh = Mesh::new(vertices, indices, material, 1);

        Self { mesh }
    }
}

impl Drawable for Ground {
    fn tick(&mut self) {
        // nothing changes
    }

    fn draw(&mut self, shader: &Shader) {
        self.mesh.draw(shader);
    }
}
