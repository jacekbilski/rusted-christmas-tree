use core::f32::consts::PI;

use cgmath::Point3;

pub fn gen_objects() -> (Vec<f32>, Vec<u32>) {
    let slices = 40 as u32;
    let colour: [f32; 3] = [0., 1., 0.];

    let mut vertices: Vec<f32> = Vec::with_capacity(9 * (slices + 1) as usize);
    let mut indices: Vec<u32> = Vec::with_capacity(3 * (slices + 1) as usize);

    // lower segment
    gen_tree_segment(slices, colour, &mut vertices, &mut indices, 4., 0, -3., 5.);

    // middle segment
    gen_tree_segment(slices, colour, &mut vertices, &mut indices, 3., 1, 0., 3.);

    // upper segment
    gen_tree_segment(slices, colour, &mut vertices, &mut indices, 2., 2, 2., 2.);

    (vertices, indices)
}

fn gen_tree_segment(slices: u32, colour: [f32; 3], vertices: &mut Vec<f32>, indices: &mut Vec<u32>, radius: f32, segment: u32, segments_bottom: f32, segments_height: f32) {
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
        vertices.extend(colour.iter());
        vertices.extend(normal.iter());

        let upper_vertex_arr: [f32; 3] = upper_vertex.into();
        vertices.extend(upper_vertex_arr.iter());
        vertices.extend(colour.iter());
        vertices.extend(normal.iter());

        if i != slices - 1 {
            indices.extend([indices_offset + 2 * i, indices_offset + 2 * i + 1, indices_offset + 2 * i + 2].iter());
        }
    }
    indices.extend([indices_offset + 2 * (slices - 1), indices_offset + 2 * (slices - 1) + 1, indices_offset].iter());
}
