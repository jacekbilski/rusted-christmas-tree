use crate::shader::Shader;

pub trait Drawable {
    fn draw(&mut self, shader: &Shader);
}
