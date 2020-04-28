use crate::shader::Shader;

pub trait Drawable {
    /// Changes the mesh from one frame to another
    fn tick(&mut self);
    fn draw(&mut self, shader: &Shader);
}
