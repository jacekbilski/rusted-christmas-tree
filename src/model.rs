use core::mem;

use cgmath::Matrix4;

use crate::shader::Shader;

pub struct Instance {
    pub model: Matrix4<f32>,
}

impl Instance {
    pub fn size() -> usize {
        mem::size_of::<Matrix4<f32>>()
    }
}

pub trait Model {
    /// Do all necessary things to advance the model to the next frame
    fn next_frame(&mut self);

    /// Draw the model using given shader
    fn draw(&mut self, shader: &Shader);
}
