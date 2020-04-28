use cgmath::{Point3, vec3};
use glfw::Window;

use crate::camera::Camera;
use crate::drawable::Drawable;
use crate::lights::Lights;
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
    drawables: Vec<Box<dyn Drawable>>,
    snow: Snow,
}

impl Scene {
    pub fn setup(window: &Window) -> Self {
        let camera = Camera::new(Point3::new(15., 12., 12.), Point3::new(0., 0., 0.), &window);
        let mut lights = Lights::setup();
        lights.add(Point3::new(10., 100., 10.), vec3(0.3, 0.3, 0.3), vec3(0.2, 0.2, 0.2), vec3(0., 0., 0.));
        lights.add(Point3::new(5., 6., 2.), vec3(0.2, 0.2, 0.2), vec3(2., 2., 2.), vec3(0.5, 0.5, 0.5));

        let shader = Shader::new("src/xmas_tree/shaders/static.vert", "src/xmas_tree/shaders/static.frag");

        let meshes = Scene::add_meshes();
        let snow = Snow::new();
        Scene { camera, lights, shader, drawables: meshes, snow }
    }

    fn add_meshes() -> Vec<Box<dyn Drawable>> {
        let mut objects: Vec<Box<dyn Drawable>> = Vec::new();
        objects.push(Box::new(Ground::new()));
        objects.push(Box::new(Tree::new()));
        objects.push(Box::new(Baubles::new()));
        objects
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            for d in &mut self.drawables {
                d.draw(&self.shader);
            }
            self.snow.draw(&self.shader);
        }
    }
}
