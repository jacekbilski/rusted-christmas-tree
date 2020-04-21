use cgmath::Point3;
use glfw::Window;

use crate::drawable::Drawable;
use crate::shader::Shader;
use crate::xmas_tree::snow::Snow;
use crate::xmas_tree::static_object::StaticObject;

mod static_object;
mod baubles;
mod ground;
mod snow;
mod tree;

pub struct XmasTree {
    shader: Shader,
    drawables: Vec<Box<dyn Drawable>>,
}

impl XmasTree {
    pub fn setup() -> Self {
        let shader = Shader::new("src/xmas_tree/shaders/static.vert", "src/xmas_tree/shaders/static.frag");
        // shader.set_light(Point3::new(10., 100., 10.), 1., 1., 1.);
        shader.set_light(Point3::new(5., 6., 2.), 1., 1., 1.);

        let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
        let ground = ground::gen_objects();
        drawables.push(Box::new(StaticObject::new(ground.0, ground.1)));
        let tree = tree::gen_objects();
        drawables.push(Box::new(StaticObject::new(tree.0, tree.1)));
        let baubles = baubles::gen_objects();
        drawables.push(Box::new(StaticObject::new(baubles.0, baubles.1)));
        drawables.push(Box::new(Snow::new()));
        XmasTree { shader, drawables }
    }
}

impl Drawable for XmasTree {
    fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader.id);
            for d in &self.drawables {
                d.draw();
            }
        }
    }
}
