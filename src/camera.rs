extern crate gl;

use std::{mem, ptr};
use std::os::raw::c_void;

use cgmath::{Deg, Matrix4, perspective, Point3, vec3, Vector4};
use cgmath::prelude::*;
use glfw::Window;

use crate::shader::CAMERA_UBO_BINDING_POINT;

pub struct Camera {
    position: Point3<f32>,
    look_at: Point3<f32>,
    ubo: u32,
    window_width: f32,
    window_height: f32,
}

impl Camera {
    pub fn new(position: Point3<f32>, look_at: Point3<f32>, window: &mut Window) -> Self {
        let (window_width, window_height) = window.get_size();
        let mut camera = Camera {position, look_at, ubo: 0, window_width: window_width as f32, window_height: window_height as f32};
        let ubo = camera.setup_camera_ubo();
        camera.ubo = ubo;
        camera.set_position(position, look_at);
        camera
    }

    fn setup_camera_ubo(&self) -> u32 {
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

    pub fn set_position(&self, position: Point3<f32>, look_at: Point3<f32>) {
        let matrix_size = mem::size_of::<Matrix4<f32>>() as isize;
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, vector3_size, position.as_ptr() as *const c_void);

            let view: Matrix4<f32> = Matrix4::look_at(position, look_at, vec3(0.0, 1.0, 0.0));
            gl::BufferSubData(gl::UNIFORM_BUFFER, vector3_size, matrix_size, view.as_ptr() as *const c_void);
            let projection = perspective(Deg(45.0), self.window_width / self.window_height, 0.1, 100.0);
            gl::BufferSubData(gl::UNIFORM_BUFFER, vector3_size + matrix_size, matrix_size, projection.as_ptr() as *const c_void);

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

}
