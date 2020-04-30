extern crate gl;

use std::{mem, ptr};
use std::os::raw::c_void;

use cgmath::{Vector3, Vector4};
use cgmath::prelude::*;

use crate::shader::MATERIALS_UBO_BINDING_POINT;

const MAX_MATERIALS: isize = 100;

pub type MaterialId = f32;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Material {
    fn size() -> isize {
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize; // there's no mistake, Vector3 takes the same amount of memory as Vector4
        3 * vector3_size
    }
}

pub struct Materials {
    ubo: u32,
    materials: Vec<Material>,
}

impl Materials {
    pub fn setup() -> Self {
        Materials { ubo: Materials::setup_lights_ubo(), materials: vec![] }
    }

    fn setup_lights_ubo() -> u32 {
        unsafe {
            let mut materials_ubo = 0 as u32;
            gl::GenBuffers(1, &mut materials_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, materials_ubo);
            gl::BufferData(gl::UNIFORM_BUFFER, MAX_MATERIALS * Material::size(), ptr::null(), gl::STATIC_DRAW);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, MATERIALS_UBO_BINDING_POINT, materials_ubo);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            materials_ubo
        }
    }

    pub fn add(&mut self, material: Material) -> MaterialId {
        self.materials.push(material);
        let material_id = self.materials.len() as usize - 1;
        unsafe {
            let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.ubo);
            gl::BufferSubData(gl::UNIFORM_BUFFER, material_id as isize * Material::size() + 0 * vector3_size, vector3_size, self.materials[material_id].ambient.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, material_id as isize * Material::size() + 1 * vector3_size, vector3_size, self.materials[material_id].diffuse.as_ptr() as *const c_void);
            gl::BufferSubData(gl::UNIFORM_BUFFER, material_id as isize * Material::size() + 2 * vector3_size, vector3_size, self.materials[material_id].specular.as_ptr() as *const c_void);
            // small hack here, shininess is not passed as a separate value, but as specular.w, 4th value in vec4
            gl::BufferSubData(gl::UNIFORM_BUFFER, material_id as isize * Material::size() + 2 * vector3_size + 12, 4, &self.materials[material_id].shininess as *const f32 as *const c_void);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
        material_id as MaterialId
    }
}
