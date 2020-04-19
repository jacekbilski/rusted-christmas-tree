pub fn gen_objects() -> (Vec<f32>, Vec<u32>) {
    let vertices: Vec<f32> = vec![
        // position, colour, normal vector
        -10., -5., -10., 1., 1., 1., 0., 1., 0., // far
        -10., -5., 10., 1., 1., 1., 0., 1., 0., // left
        10., -5., -10., 1., 1., 1., 0., 1., 0., // right
        10., -5., 10., 1., 1., 1., 0., 1., 0., // near
    ];
    let indices: Vec<u32> = vec![
        0, 2, 1,
        1, 2, 3,
    ];

    (vertices, indices)
}
