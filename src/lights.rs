extern crate gl;

use std::{mem, ptr};
use std::os::raw::c_void;

use cgmath::{Point3, vec3, Vector3, Vector4};
use cgmath::prelude::*;

use crate::shader::LIGHTS_UBO_BINDING_POINT;

struct Light {
    position: Point3<f32>,
    ambient: Vector3<f32>,
    diffuse: Vector3<f32>,
    specular: Vector3<f32>,
}

pub struct Lights {
    ubo: u32,
    lights: Vec<Light>,
}

impl Lights {
    pub fn setup() -> Self {
        Lights { ubo: Lights::setup_lights_ubo(), lights: vec![] }
    }

    fn setup_lights_ubo() -> u32 {
        unsafe {
            let mut light_ubo = 0 as u32;
            gl::GenBuffers(1, &mut light_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, light_ubo);
            let vector3_size = mem::size_of::<Vector4<f32>>() as isize; // there's no mistake, Vector3 takes the same amount of memory as Vector4
            gl::BufferData(gl::UNIFORM_BUFFER, 4 * vector3_size, ptr::null(), gl::STATIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, LIGHTS_UBO_BINDING_POINT, light_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            light_ubo
        }
    }

    pub fn add(&mut self, position: Point3<f32>, ambient: Vector3<f32>, diffuse: Vector3<f32>, specular: Vector3<f32>) {
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
        let light = Light {position, ambient, diffuse, specular};
        self.lights  = vec![light]; // so far only one is supported
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, vector3_size, position.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 1 * vector3_size, vector3_size, ambient.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 2 * vector3_size, vector3_size, diffuse.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 3 * vector3_size, vector3_size, specular.as_ptr() as *const c_void);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}
