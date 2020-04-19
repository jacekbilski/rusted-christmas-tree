use cgmath::{Deg, Matrix4, perspective, Point3, vec3};
use cgmath::prelude::*;
use glfw::Window;

use crate::drawable::Drawable;
use crate::shader::Shader;
use crate::xmas_tree::static_object::StaticObject;

mod static_object;
mod baubles;
mod ground;
mod tree;

pub struct XmasTree {
    shader: Shader,
    drawables: Vec<Box<dyn Drawable>>,
}

impl XmasTree {
    pub fn setup() -> Self {
        let shader = Shader::new("src/xmas_tree/shaders/static.vert", "src/xmas_tree/shaders/static.frag");
        let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
        let ground = ground::gen_objects();
        drawables.push(Box::new(StaticObject::new(ground.0, ground.1)));
        let tree = tree::gen_objects();
        drawables.push(Box::new(StaticObject::new(tree.0, tree.1)));
        let baubles = baubles::gen_objects();
        drawables.push(Box::new(StaticObject::new(baubles.0, baubles.1)));
        XmasTree { shader, drawables }
    }
}

impl Drawable for XmasTree {
    fn draw(&self, window: &mut Window) {
        unsafe {
            let camera_position = Point3::new(15., 12., 12.);
            gl::UseProgram(self.shader.id);
            self.shader.set_vec3("lightColour", &vec3(1., 1., 1.));
            // self.shader.set_vec3("lightPosition", &vec3(10., 100., 10.));
            self.shader.set_vec3("lightPosition", &vec3(5., 6., 2.));
            self.shader.set_vec3("cameraPosition", &camera_position.to_vec());

            self.shader.set_mat4("model", &Matrix4::identity() as &Matrix4<f32>);
            let view: Matrix4<f32> = Matrix4::look_at(camera_position, Point3::new(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
            self.shader.set_mat4("view", &view);
            let (width, height) = window.get_size();
            let projection = perspective(Deg(45.0), width as f32 / height as f32, 0.1, 100.0);
            self.shader.set_mat4("projection", &projection);

            for d in &self.drawables {
                d.draw(window);
            }
        }
    }
}
