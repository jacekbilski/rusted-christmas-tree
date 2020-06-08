use cgmath::{Matrix4, Point3, SquareMatrix, vec3, Vector3};

use crate::material::{Material, Materials};
use crate::model::{Instance, Model};
use crate::shader::Shader;
use crate::xmas_tree::mesh::{Mesh, Vertex};

pub struct Ground {
    mesh: Mesh,
}

impl Ground {
    pub fn new(materials: &mut Materials) -> Self {
        let vertices: Vec<Vertex> = vec![
            Vertex { position: Point3::new(-10., -5., -10.), normal: vec3(0., 1., 0.) },   // far
            Vertex { position: Point3::new(-10., -5., 10.), normal: vec3(0., 1., 0.) }, // left
            Vertex { position: Point3::new(10., -5., -10.), normal: vec3(0., 1., 0.) }, // right
            Vertex { position: Point3::new(10., -5., 10.), normal: vec3(0., 1., 0.) }, // near
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,
            1, 3, 2,
        ];

        let ambient: Vector3<f32> = vec3(1., 1., 1.);
        let diffuse: Vector3<f32> = vec3(0.623960, 0.686685, 0.693872);
        let specular: Vector3<f32> = vec3(0.5, 0.5, 0.5);
        let shininess: f32 = 225.;
        let material = Material { ambient, diffuse, specular, shininess };
        let material_id = materials.add(material);

        let mesh = Mesh::new(vertices, indices, 1);
        mesh.fill_instances_vbo(&vec![Instance { model: Matrix4::identity(), material_id }]);
        Self { mesh }
    }
}

impl Model for Ground {
    fn next_frame(&mut self) {
        // nothing changes
    }

    fn draw(&mut self, shader: &Shader) {
        self.mesh.draw_single(shader);
    }
}
