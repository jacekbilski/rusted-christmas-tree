extern crate gl;

use std::{mem, ptr};
use std::os::raw::c_void;

use cgmath::{Point3, vec3, Vector3, Vector4};
use cgmath::prelude::*;

use crate::shader::LIGHTS_UBO_BINDING_POINT;

pub struct Lights {
    ubo: u32,
    position: Vec<Point3<f32>>,
    colour: Vec<Vector3<f32>>,
}

impl Lights {
    pub fn setup() -> Self {
        Lights { ubo: Lights::setup_lights_ubo(), position: vec![], colour: vec![] }
    }

    fn setup_lights_ubo() -> u32 {
        unsafe {
            let mut light_ubo = 0 as u32;
            gl::GenBuffers(1, &mut light_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, light_ubo);
            let vector3_size = mem::size_of::<Vector4<f32>>() as isize; // there's no mistake, Vector3 takes the same amount of memory as Vector4
            gl::BufferData(gl::UNIFORM_BUFFER, 2 * vector3_size, ptr::null(), gl::STATIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, LIGHTS_UBO_BINDING_POINT, light_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            light_ubo
        }
    }

    pub fn add(&mut self, position: Point3<f32>, red: f32, green: f32, blue: f32) {
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
        let colour: Vector3<f32> = vec3(red, green, blue);
        self.position.push(position);
        self.colour.push(colour);
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, vector3_size, position.as_ptr() as *const c_void);

            gl::BufferSubData(gl::UNIFORM_BUFFER, vector3_size, vector3_size, colour.as_ptr() as *const c_void);

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}
