use core::f32::consts::PI;

use cgmath::{Point3, vec3, Vector3};
use tobj;

use crate::material::Material;

pub fn gen_objects_old() -> (Vec<f32>, Vec<u32>, Material) {
    let slices = 40 as u32;

    let mut vertices: Vec<f32> = Vec::with_capacity(6 * (slices + 1) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity(3 * (slices + 1) as usize);

    // lower segment
    gen_tree_segment(slices, &mut vertices, &mut indices, 4., 0, -3., 5.);

    // middle segment
    gen_tree_segment(slices, &mut vertices, &mut indices, 3., 1, 0., 3.);

    // upper segment
    gen_tree_segment(slices, &mut vertices, &mut indices, 2., 2, 2., 2.);

    let ambient: Vector3<f32> = vec3(0.02, 0.35, 0.01);
    let diffuse: Vector3<f32> = vec3(0.02, 0.35, 0.01);
    let specular: Vector3<f32> = vec3(0.1, 0.1, 0.1);
    let shininess: f32 = 225.;
    let material = Material { ambient, diffuse, specular, shininess };

    (vertices, indices, material)
}

fn gen_tree_segment(slices: u32, vertices: &mut Vec<f32>, indices: &mut Vec<u32>, radius: f32, segment: u32, segments_bottom: f32, segments_height: f32) {
    let angle_diff = PI * 2. / slices as f32;
    let indices_offset = 2 * segment * slices;
    for i in 0..slices {
        let angle = angle_diff * i as f32;

        let bottom_vertex = Point3::new(radius * angle.sin(), segments_bottom, radius * angle.cos());
        let upper_vertex = Point3::new(0., segments_bottom + segments_height, 0.);
        let next_bottom_vertex = Point3::new(radius * (angle + angle_diff).sin(), segments_bottom, radius * (angle + angle_diff).cos());
        let vec_1 = next_bottom_vertex - bottom_vertex;
        let vec_2 = upper_vertex - bottom_vertex;
        let normal: [f32; 3] = vec_1.cross(vec_2).into();

        let bottom_vertex_arr: [f32; 3] = bottom_vertex.into();
        vertices.extend(bottom_vertex_arr.iter());
        vertices.extend(normal.iter());

        let upper_vertex_arr: [f32; 3] = upper_vertex.into();
        vertices.extend(upper_vertex_arr.iter());
        vertices.extend(normal.iter());

        if i != slices - 1 {
            indices.extend([indices_offset + 2 * i, indices_offset + 2 * i + 1, indices_offset + 2 * i + 2].iter());
        }
    }
    indices.extend([indices_offset + 2 * (slices - 1), indices_offset + 2 * (slices - 1) + 1, indices_offset].iter());
}

pub fn gen_objects() -> (Vec<f32>, Vec<u32>, Material) {
    let tree = tobj::load_obj("tree.obj");
    let (models, materials) = tree.unwrap();
    for i in 0..models.len() {
        println!("Found model, i: {}, name: '{}'", i, models[i].name);
    }
    let vertices_count: usize = models.iter().map(|m| m.mesh.positions.len()).sum::<usize>() / 3;
    let indices_count: usize = models.iter().map(|m| m.mesh.indices.len()).sum::<usize>() / 3;
    let mut vertices: Vec<f32> = Vec::with_capacity(9 * vertices_count);
    let mut indices: Vec<u32> = Vec::with_capacity(3 * indices_count as usize);
    for mi in 0..models.len() {
        let mesh = models[mi].mesh.clone();
        for vi in (0..mesh.positions.len()).step_by(3) {
            vertices.push(mesh.positions[vi]);
            vertices.push(mesh.positions[vi+1]);
            vertices.push(mesh.positions[vi+2]);
            vertices.push(mesh.normals[vi]);
            vertices.push(mesh.normals[vi+1]);
            vertices.push(mesh.normals[vi+2]);
        }
        indices.extend(mesh.indices.iter());
    }
    let material = Material{ambient: Vector3::from(materials[2].ambient), diffuse: Vector3::from(materials[2].diffuse), specular: Vector3::from(materials[2].specular), shininess: materials[2].shininess};
    (vertices, indices, material)
}
