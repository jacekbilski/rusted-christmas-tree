use cgmath::{vec3, Vector3};

use crate::material::Material;

pub fn gen_objects() -> (Vec<f32>, Vec<u32>, Material) {
    let vertices: Vec<f32> = vec![
        // position, normal vector
        -10., -5., -10., 0., 1., 0., // far
        -10., -5., 10., 0., 1., 0., // left
        10., -5., -10., 0., 1., 0., // right
        10., -5., 10., 0., 1., 0., // near
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

    (vertices, indices, material)
}
