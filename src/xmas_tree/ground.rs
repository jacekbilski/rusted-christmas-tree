extern crate gl;

use std::ptr;

use glfw::Window;

use crate::drawable::Drawable;
use crate::xmas_tree::static_object::StaticObject;

type VAO = u32;

pub struct Ground {
    vao: VAO,
    indices: Vec<u32>,
}

impl Ground {
    pub fn setup() -> Self {
        let (vertices, indices) = Self::gen_vertices();
        let within_vao = || {
            Self::create_vbo(&vertices);
            Self::create_ebo(&indices);
        };
        let vao = Self::create_vao(within_vao);

        Self { vao, indices }
    }

    fn gen_vertices() -> (Vec<f32>, Vec<u32>) {
        let vertices: Vec<f32> = vec![ // position, colour, normal vector
           -10., -5., -10., 1., 1., 1., 0., 1., 0., // far
           -10., -5., 10., 1., 1., 1., 0., 1., 0., // left
           10., -5., -10., 1., 1., 1., 0., 1., 0., // right
           10., -5., 10., 1., 1., 1., 0., 1., 0., // near
        ];
        let indices: Vec<u32> = vec![
            0, 2, 1,
            2, 1, 3,
        ];

        (vertices, indices)
    }
}

impl Drawable for Ground {
    fn draw(&self, _window: &mut Window) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl StaticObject for Ground {

}
