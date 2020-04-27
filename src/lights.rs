extern crate gl;

use std::{mem, ptr};
use std::os::raw::c_void;

use cgmath::{Point3, Vector3, Vector4};
use cgmath::prelude::*;

use crate::shader::LIGHTS_UBO_BINDING_POINT;

use self::gl::types::*;

const MAX_LIGHTS: u8 = 4;

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
            gl::BufferData(gl::UNIFORM_BUFFER, 16 + MAX_LIGHTS as isize * 4 * vector3_size, ptr::null(), gl::STATIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, LIGHTS_UBO_BINDING_POINT, light_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            light_ubo
        }
    }

    pub fn add(&mut self, position: Point3<f32>, ambient: Vector3<f32>, diffuse: Vector3<f32>, specular: Vector3<f32>) {
        let light = Light {position, ambient, diffuse, specular};
        self.lights.push(light);
        let lights_no= self.lights.len() as isize;
        unsafe {
            let int_size = mem::size_of::<GLint>() as isize;
            let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
            let light_size = 4 * vector3_size;
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, int_size, &lights_no as *const isize as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 0 * vector3_size, vector3_size, position.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 1 * vector3_size, vector3_size, ambient.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 2 * vector3_size, vector3_size, diffuse.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 16 + (lights_no - 1) * light_size + 3 * vector3_size, vector3_size, specular.as_ptr() as *const c_void);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}
