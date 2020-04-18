use core::f32::consts::PI;

use cgmath::Point3;

pub fn gen_vertices() -> (Vec<f32>, Vec<u32>) {
    let slices = 40 as u32;
    let colour: [f32; 3] = [1., 0.0, 0.0];

    let mut vertices: Vec<f32> = Vec::with_capacity(9 * (slices + 1) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity(3 * (slices + 1) as usize);

    let center = Point3::new(0., 4.2, 0.);
    gen_sphere(center, 0.2, 2, &colour, &mut vertices, &mut indices);

    // println!("Vertices: {:?}", &vertices);
    // println!("Indices: {:?}", &indices);
    (vertices, indices)
}

fn gen_sphere(center: Point3<f32>, radius: f32, precision: u8, colour: &[f32; 3], vertices: &mut Vec<f32>, indices: &mut Vec<u32>) {
    let angle_diff = PI / precision as f32;

    // first layer is special, it's built out of triangles, not trapezoids
    for slice in 0..2 * precision {
        vertices.extend([center.x, center.y + radius, center.z].iter());
        vertices.extend(colour.iter());
        vertices.extend([0., 1., 0.].iter());
    }
    // no indices yet, I'm adding them after adding all vertices from a given layer

    for layer in 1..precision {
        // let v_angle = angle_diff * layer as f32;   // vertically I'm doing only half rotation
        for slice in 0..(2 * precision) {
            let h_angle = angle_diff * slice as f32;   // horizontally I'm doing full circle
            let vertex = Point3::new(center.x + radius * h_angle.sin(), center.y, center.z + radius * h_angle.cos());

            let vertex_arr: [f32; 3] = vertex.into();
            vertices.extend(vertex_arr.iter());
            vertices.extend(colour.iter());
            vertices.extend([h_angle.sin(), 0., h_angle.cos()].iter());
        }

        // for slice in 0..2 * precision - 1 {
        indices.extend([0, 4, 5].iter());
        indices.extend([1, 5, 6].iter());
        indices.extend([2, 6, 7].iter());
        // }
        indices.extend([3, 7, 4].iter());
    }

    // last layer is also special, it's built out of triangles, not trapezoids
    for slice in 0..2 * precision {
        vertices.extend([center.x, center.y - radius, center.z].iter());
        vertices.extend(colour.iter());
        vertices.extend([0., -1., 0.].iter());
    }
    // for slice in 0..2*precision {
    // }
    indices.extend([4, 5, 8].iter());
    indices.extend([5, 6, 9].iter());
    indices.extend([6, 7, 10].iter());
    indices.extend([7, 4, 11].iter());
}
