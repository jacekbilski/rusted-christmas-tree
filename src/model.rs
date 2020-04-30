use core::mem;

use cgmath::Matrix4;

use crate::material::MaterialId;
use crate::shader::Shader;

#[derive(Debug)]
#[repr(C)]  // to make sure memory representation is like in the code
pub struct Instance {
    pub model: Matrix4<f32>,
    pub material_id: MaterialId,
}

impl Instance {
    pub fn size() -> usize {
        mem::size_of::<Matrix4<f32>>() + mem::size_of::<u32>()
    }
}

pub trait Model {
    /// Do all necessary things to advance the model to the next frame
    fn next_frame(&mut self);

    /// Draw the model using given shader
    fn draw(&mut self, shader: &Shader);
}
