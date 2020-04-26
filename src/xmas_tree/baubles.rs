use core::f32::consts::PI;

use cgmath::{Point3, vec3, Vector3};

use crate::material::Material;

struct Bauble {
    center: Point3<f32>,
    colour: [f32; 3],
}

pub fn gen_objects() -> (Vec<f32>, Vec<u32>, Material) {
    let precision = 8 as u32;
    let radius = 0.2 as f32;

    let red: [f32; 3] = [1., 0., 0.];
    let yellow: [f32; 3] = [1., 1., 0.];
    let blue: [f32; 3] = [0., 0., 1.];
    let light_blue: [f32; 3] = [0., 1., 1.];
    let violet: [f32; 3] = [1., 0., 1.];

    let baubles: [Bauble; 16] = [
        Bauble { center: Point3::new(0., 4.2, 0.), colour: red },
        Bauble { center: Point3::new(1., 3., 1.), colour: yellow },
        Bauble { center: Point3::new(1.0, 1.0, 2.0), colour: light_blue },
        Bauble { center: Point3::new(-1.0, 1.0, 2.0), colour: blue },
        Bauble { center: Point3::new(1.0, 1.0, -2.0), colour: violet },
        Bauble { center: Point3::new(2.0, 1.0, 0.0), colour: red },
        Bauble { center: Point3::new(3.0, -1.0, 0.0), colour: blue },
        Bauble { center: Point3::new(0.0, -1.0, 3.0), colour: yellow },
        Bauble { center: Point3::new(-3.0, -1.0, 0.0), colour: red },
        Bauble { center: Point3::new(0.0, -1.0, -3.0), colour: blue },
        Bauble { center: Point3::new(2.0, -2.0, -3.0), colour: blue },
        Bauble { center: Point3::new(2.0, -2.0, 3.0), colour: violet },
        Bauble { center: Point3::new(3.0, -2.0, -2.0), colour: violet },
        Bauble { center: Point3::new(-3.0, -2.0, -2.0), colour: red },
        Bauble { center: Point3::new(0.0, -2.0, 4.0), colour: red },
        Bauble { center: Point3::new(4.0, -2.0, 0.0), colour: yellow }
    ];

    let mut vertices: Vec<f32> = Vec::with_capacity(9 * 2 * precision.pow(2) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity(3 * 4 * precision.pow(2) as usize);

    for i in 0..baubles.len() {
        gen_sphere(&mut vertices, &mut indices, baubles[i].center, radius, precision, &baubles[i].colour);
    }

    let ambient: Vector3<f32> = vec3(0.1745, 0.01175, 0.01175);
    let diffuse: Vector3<f32> = vec3(0.61424, 0.04136, 0.04136);
    let specular: Vector3<f32> = vec3(0.727811, 0.626959, 0.626959);
    let shininess: f32 = 76.8;
    let material = Material { ambient, diffuse, specular, shininess };

    (vertices, indices, material)
}

fn gen_sphere(vertices: &mut Vec<f32>, indices: &mut Vec<u32>, center: Point3<f32>, radius: f32, precision: u32, colour: &[f32; 3]) {
    let vertices_offset = vertices.len() / 9;
    gen_vertices(vertices, center, radius, precision, colour);
    gen_indices(indices, precision, vertices_offset)
}

fn gen_vertices(vertices: &mut Vec<f32>, center: Point3<f32>, radius: f32, precision: u32, colour: &[f32; 3]) {
    let angle_diff = PI / precision as f32;

    vertices.extend([center.x, center.y + radius, center.z].iter());
    vertices.extend(colour.iter());
    vertices.extend([0., 1., 0.].iter());

    for layer in 1..precision {
        let v_angle = angle_diff * layer as f32;   // vertically I'm doing only half rotation
        for slice in 0..(2 * precision) {
            let h_angle = angle_diff * slice as f32;   // horizontally I'm doing full circle
            let layer_radius = radius * v_angle.sin();
            let vertex = Point3::new(center.x + layer_radius * h_angle.sin(), center.y + radius * v_angle.cos(), center.z + layer_radius * h_angle.cos());

            let vertex_arr: [f32; 3] = vertex.into();
            vertices.extend(vertex_arr.iter());
            vertices.extend(colour.iter());
            vertices.extend([h_angle.sin(), v_angle.cos(), h_angle.cos()].iter());
        }
    }

    vertices.extend([center.x, center.y - radius, center.z].iter());
    vertices.extend(colour.iter());
    vertices.extend([0., -1., 0.].iter());
}

fn gen_indices(indices: &mut Vec<u32>, precision: u32, vertices_offset: usize) {
    let find_index = |layer: u32, slice: u32| {
        // layers 0 and precision have only 1 vertex
        if layer == 0 {
            vertices_offset as u32
        } else if layer == precision {
            vertices_offset as u32 + (layer - 1) * 2 * precision + 1
        } else {
            vertices_offset as u32 + (layer - 1) * 2 * precision + 1 + slice % (2 * precision)
        }
    };

    // I'm generating indices for triangles drawn between this and previous layers of vertices
    let mut layer = 1;
    for slice in 0..2 * precision {
        // first layer has only triangles
        indices.extend([find_index(layer - 1, slice), find_index(layer, slice + 1), find_index(layer, slice)].iter());
    }

    for layer in 2..precision {
        for slice in 0..2 * precision {
            // midddle layers are actually traapezoids, I need two triangles per slice
            indices.extend([find_index(layer - 1, slice), find_index(layer, slice + 1), find_index(layer, slice)].iter());
            indices.extend([find_index(layer - 1, slice), find_index(layer - 1, slice + 1), find_index(layer, slice + 1)].iter());
        }
    }

    layer = precision;
    for slice in 0..2 * precision {
        // last layer has only triangles
        indices.extend([find_index(layer - 1, slice), find_index(layer - 1, slice + 1), find_index(layer, slice)].iter());
    }
}
