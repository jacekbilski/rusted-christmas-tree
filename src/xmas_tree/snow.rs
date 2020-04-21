use core::f32::consts::PI;

pub fn gen_objects() -> (Vec<f32>, Vec<u32>) {
    let radius: f32 = 0.07;
    let colour: [f32; 3] = [1., 1., 1.];
    let normal: [f32; 3] = [1., 0., 0.];
    let mut vertices: Vec<f32> = vec![];

    let angle_diff = PI / 3 as f32;

    for i in 0..6 {
        let angle = i as f32 * angle_diff;
        vertices.extend([0., radius * angle.cos(), radius * angle.sin()].iter());
        vertices.extend(colour.iter());
        vertices.extend(normal.iter());
    }
    let indices: Vec<u32> = vec![
        4, 2, 0,
        5, 3, 1,
    ];

    (vertices, indices)
}
