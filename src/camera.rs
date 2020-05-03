extern crate gl;

use std::{mem, ptr};
use std::os::raw::c_void;

use cgmath::{Deg, Matrix4, perspective, Point3, vec3, Vector4};
use cgmath::prelude::*;
use glfw::Window;

use crate::coords::SphericalPoint3;
use crate::shader::CAMERA_UBO_BINDING_POINT;

pub struct Camera {
    position: SphericalPoint3<f32>,
    look_at: Point3<f32>,
    ubo: u32,
    window_width: f32,
    window_height: f32,
}

impl Camera {
    pub fn new(position: SphericalPoint3<f32>, look_at: Point3<f32>, window: &Window) -> Self {
        let (window_width, window_height) = window.get_size();
        let ubo = Camera::setup_camera_ubo();
        let camera = Camera { position, look_at, ubo, window_width: window_width as f32, window_height: window_height as f32 };
        camera.update_uniforms();
        camera
    }

    fn setup_camera_ubo() -> u32 {
        unsafe {
            let mut camera_ubo = 0 as u32;
            gl::GenBuffers(1, &mut camera_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, camera_ubo);
            let matrix_size = mem::size_of::<Matrix4<f32>>() as isize;
            let vector3_size = mem::size_of::<Vector4<f32>>() as isize; // thhere's no mistake, Vector3 takes the same amount of memory as Vector4
            gl::BufferData(gl::UNIFORM_BUFFER, vector3_size + 2 * matrix_size, ptr::null(), gl::STATIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, CAMERA_UBO_BINDING_POINT, camera_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            camera_ubo
        }
    }

    fn update_uniforms(&self) {
        let matrix_size = mem::size_of::<Matrix4<f32>>() as isize;
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            let pos: Point3<f32> = self.position.into();
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, vector3_size, pos.as_ptr() as *const c_void);

            let view: Matrix4<f32> = Matrix4::look_at(self.position.into(), self.look_at, vec3(0.0, 1.0, 0.0));
            gl::BufferSubData(gl::UNIFORM_BUFFER, vector3_size, matrix_size, view.as_ptr() as *const c_void);
            let projection = perspective(Deg(45.0), self.window_width / self.window_height, 0.1, 100.0);
            gl::BufferSubData(gl::UNIFORM_BUFFER, vector3_size + matrix_size, matrix_size, projection.as_ptr() as *const c_void);

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn on_window_resize(&mut self, window: &Window) {
        let (window_width, window_height) = window.get_size();
        self.window_width = window_width as f32;
        self.window_height = window_height as f32;
        self.update_uniforms();
    }

    pub fn rotate_horizontally(&mut self, angle: f32) {
        self.position.phi += angle;
        self.update_uniforms();
    }

    pub fn rotate_vertically(&mut self, angle: f32) {
        self.position.theta += angle;
        self.update_uniforms();
    }
}
