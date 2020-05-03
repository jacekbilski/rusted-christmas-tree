use core::f32::consts::PI;

use cgmath::{Matrix4, Point3, SquareMatrix, vec3, Vector3};

use crate::material::{Material, Materials};
use crate::model::{Instance, Model};
use crate::shader::Shader;
use crate::xmas_tree::mesh::{Mesh, Vertex};

pub struct Tree {
    meshes: Vec<Mesh>,
}

impl Tree {
    pub fn new(materials: &mut Materials) -> Self {
        Self::manual(materials)
        // Self::from_model(materials)
    }

    fn manual(materials: &mut Materials) -> Self {
        let slices = 40 as u32;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(3 * 2 * slices as usize);
        let mut indices: Vec<u32> = Vec::with_capacity(3 * (slices + 1) as usize);

        // lower segment
        Self::gen_tree_segment(slices, &mut vertices, &mut indices, 4., 0, -3., 5.);

        // middle segment
        Self::gen_tree_segment(slices, &mut vertices, &mut indices, 3., 1, 0., 3.);

        // upper segment
        Self::gen_tree_segment(slices, &mut vertices, &mut indices, 2., 2, 2., 2.);

        let ambient: Vector3<f32> = vec3(0.02, 0.35, 0.01);
        let diffuse: Vector3<f32> = vec3(0.02, 0.35, 0.01);
        let specular: Vector3<f32> = vec3(0.1, 0.1, 0.1);
        let shininess: f32 = 225.;
        let material = Material { ambient, diffuse, specular, shininess };
        let material_id = materials.add(material);

        let mesh = Mesh::new(vertices, indices, 1);
        mesh.fill_instances_vbo(&vec![Instance { model: Matrix4::identity(), material_id }]);
        Self { meshes: vec![mesh] }
    }

    fn gen_tree_segment(slices: u32, vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>, radius: f32, segment: u32, segments_bottom: f32, segments_height: f32) {
        let angle_diff = PI * 2. / slices as f32;
        let indices_offset = 2 * segment * slices;
        for i in 0..slices {
            let angle = angle_diff * i as f32;

            let bottom_vertex = Point3::new(radius * angle.sin(), segments_bottom, radius * angle.cos());
            let upper_vertex = Point3::new(0., segments_bottom + segments_height, 0.);
            let next_bottom_vertex = Point3::new(radius * (angle + angle_diff).sin(), segments_bottom, radius * (angle + angle_diff).cos());
            let vec_1 = next_bottom_vertex - bottom_vertex;
            let vec_2 = upper_vertex - bottom_vertex;
            let normal = vec_1.cross(vec_2);
            vertices.push(Vertex { position: bottom_vertex, normal });
            vertices.push(Vertex { position: upper_vertex, normal });

            if i != slices - 1 {
                indices.extend([indices_offset + 2 * i, indices_offset + 2 * i + 1, indices_offset + 2 * i + 2].iter());
            }
        }
        indices.extend([indices_offset + 2 * (slices - 1), indices_offset + 2 * (slices - 1) + 1, indices_offset].iter());
    }

    fn from_model(materials: &mut Materials) -> Self {
        let tree = tobj::load_obj("models/tree.obj");
        let (models, model_materials) = tree.unwrap();
        for i in 0..models.len() {
            println!("Found model, i: {}, name: '{}'", i, models[i].name);
        }
        let vertices_count: usize = models.iter().map(|m| m.mesh.positions.len()).sum::<usize>() / 3;
        let indices_count: usize = models.iter().map(|m| m.mesh.indices.len()).sum::<usize>() / 3;
        let mut vertices: Vec<Vertex> = Vec::with_capacity(vertices_count);
        let mut indices: Vec<u32> = Vec::with_capacity(3 * indices_count as usize);
        // for mi in 0..models.len() {
        let mi = 0;
            let mesh = models[mi].mesh.clone();
            for vi in (0..mesh.positions.len()).step_by(3) {
                let position = Point3::new(mesh.positions[vi], mesh.positions[vi+1], mesh.positions[vi+2]);
                let normal = vec3(mesh.normals[vi], mesh.normals[vi+1], mesh.normals[vi+2]);
                vertices.push(Vertex { position, normal });
            }
            indices.extend(mesh.indices.iter());

        // }
        let material = Material{ambient: Vector3::from(model_materials[2].ambient), diffuse: Vector3::from(model_materials[2].diffuse), specular: Vector3::from(model_materials[2].specular), shininess: model_materials[2].shininess};
        let material_id = materials.add(material);

        let mesh = Mesh::new(vertices, indices, 1);
        mesh.fill_instances_vbo(&vec![Instance { model: Matrix4::identity(), material_id }]);
        Self { meshes: vec![mesh] }
    }
}

impl Model for Tree {
    fn next_frame(&mut self) {
        // nothing changes
    }

    fn draw(&mut self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw_single(shader);
        }
    }
}
