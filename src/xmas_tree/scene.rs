use cgmath::{Point3, vec3};
use glfw::Window;

use crate::camera::Camera;
use crate::drawable::Drawable;
use crate::lights::Lights;
use crate::shader::Shader;
use crate::xmas_tree::{baubles, ground, tree};
use crate::xmas_tree::mesh::Mesh;
use crate::xmas_tree::snow::Snow;

pub struct Scene {
    pub camera: Camera,
    lights: Lights,
    shader: Shader,
    meshes: Vec<Mesh>,
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
        Scene { camera, lights, shader, meshes, snow }
    }

    fn add_meshes() -> Vec<Mesh> {
        let mut objects: Vec<Mesh> = Vec::new();
        let ground = ground::gen_objects();
        objects.push(Mesh::new(ground.0, ground.1, ground.2, 1));
        let tree = tree::gen_objects();
        objects.push(Mesh::new(tree.0, tree.1, tree.2, 1));
        let baubles = baubles::gen_objects();
        objects.push(Mesh::new(baubles.0, baubles.1, baubles.2, 1));
        objects
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            for d in &mut self.meshes {
                d.draw(&self.shader);
            }
            self.snow.draw(&self.shader);
        }
    }
}
