use core::f32::consts::PI;

use cgmath::{Point3, vec3, Vector3};

use crate::material::Material;
use crate::xmas_tree::static_object::Vertex;

pub fn gen_objects() -> (Vec<Vertex>, Vec<u32>, Material) {
    let slices = 40 as u32;

    let mut vertices: Vec<Vertex> = Vec::with_capacity(3 * 2 * slices as usize);
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
