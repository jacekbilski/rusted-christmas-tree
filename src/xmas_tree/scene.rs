use cgmath::{Point3, vec3};
use glfw::Window;

use crate::camera::Camera;
use crate::lights::Lights;
use crate::model::Model;
use crate::shader::Shader;
use crate::xmas_tree::baubles::Baubles;
use crate::xmas_tree::ground::Ground;
use crate::xmas_tree::mesh::Mesh;
use crate::xmas_tree::snow::Snow;
use crate::xmas_tree::tree::Tree;

pub struct Scene {
    pub camera: Camera,
    lights: Lights,
    shader: Shader,
    models: Vec<Box<dyn Model>>,
    snow: Snow,
}

impl Scene {
    pub fn setup(window: &Window) -> Self {
        let camera = Camera::new(Point3::new(15., 12., 12.), Point3::new(0., 0., 0.), &window);
        let mut lights = Lights::setup();
        lights.add(Point3::new(10., 100., 10.), vec3(0.3, 0.3, 0.3), vec3(0.2, 0.2, 0.2), vec3(0., 0., 0.));
        lights.add(Point3::new(5., 6., 2.), vec3(0.2, 0.2, 0.2), vec3(2., 2., 2.), vec3(0.5, 0.5, 0.5));

        let shader = Shader::new("src/xmas_tree/shaders/static.vert", "src/xmas_tree/shaders/static.frag");

        let models = Scene::add_models();
        let snow = Snow::new();
        Scene { camera, lights, shader, models, snow }
    }

    fn add_models() -> Vec<Box<dyn Model>> {
        let mut models: Vec<Box<dyn Model>> = Vec::new();
        models.push(Box::new(Ground::new()));
        models.push(Box::new(Tree::new()));
        models.push(Box::new(Baubles::new()));
        models
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            for d in &mut self.models {
                d.draw(&self.shader);
            }
            self.snow.draw(&self.shader);
        }
    }
}
