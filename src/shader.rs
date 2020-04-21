extern crate gl;

use std::{mem, ptr};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::os::raw::c_void;
use std::str;

use cgmath::{Point3, vec3, Vector3, Vector4};
use cgmath::prelude::*;

use self::gl::types::*;

pub const CAMERA_UBO_BINDING_POINT: u32 = 0;

pub struct Shader {
    pub id: u32,
    light_ubo: u32,
}

#[derive(Debug)]
enum ShaderType {
    VertexShader,
    FragmentShader,
    Program,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        unsafe {
            let shader_program = gl::CreateProgram();
            let mut shader = Shader {id: shader_program, light_ubo: 0};
            let vertex_shader = shader.add_vertex_shader(vertex_path);
            let fragment_shader = shader.add_fragment_shader(fragment_path);
            gl::LinkProgram(shader_program);
            ensure_compilation_success(ShaderType::Program, shader_program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader.bind_camera_ubo();
            shader.light_ubo = Shader::setup_light_ubo(shader_program);
            shader
        }
    }

    unsafe fn bind_camera_ubo(&self) {
        let c_name = CString::new("Camera").unwrap();
        let uniform_block_index = gl::GetUniformBlockIndex(self.id, c_name.as_ptr());
        gl::UniformBlockBinding(self.id, uniform_block_index, CAMERA_UBO_BINDING_POINT);
    }

    unsafe fn setup_light_ubo(shader_program: u32) -> u32 {
        let c_name = CString::new("Light").unwrap();
        let uniform_block_index = gl::GetUniformBlockIndex(shader_program, c_name.as_ptr());
        gl::UniformBlockBinding(shader_program, uniform_block_index, 1);
        let mut light_ubo = 0 as u32;
        gl::GenBuffers(1, &mut light_ubo);
        gl::BindBuffer(gl::UNIFORM_BUFFER, light_ubo);
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize; // there's no mistake, Vector3 takes the same amount of memory as Vector4
        gl::BufferData(gl::UNIFORM_BUFFER, 2 * vector3_size, ptr::null(), gl::STATIC_DRAW);
        gl::BindBufferBase(gl::UNIFORM_BUFFER, 1, light_ubo);
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        light_ubo
    }

    fn add_vertex_shader(&self, path: &str) -> u32 {
        let shader_source = load_from_file(path);
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &shader_source.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            ensure_compilation_success(ShaderType::VertexShader, vertex_shader);
            gl::AttachShader(self.id, vertex_shader);
            vertex_shader
        }
    }

    fn add_fragment_shader(&self, path: &str) -> u32 {
        let shader_source = load_from_file(path);
        unsafe {
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &shader_source.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            ensure_compilation_success(ShaderType::FragmentShader, fragment_shader);
            gl::AttachShader(self.id, fragment_shader);
            fragment_shader
        }
    }

    pub fn set_light(&self, position: Point3<f32>, red: f32, green: f32, blue: f32) {
        let vector3_size = mem::size_of::<Vector4<f32>>() as isize;
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.light_ubo);
            gl::BufferSubData(gl::UNIFORM_BUFFER, 0, vector3_size, position.as_ptr() as *const c_void);

            let colour: Vector3<f32> = vec3(red, green, blue);
            gl::BufferSubData(gl::UNIFORM_BUFFER, vector3_size, vector3_size, colour.as_ptr() as *const c_void);

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }
}

fn load_from_file(path: &str) -> CString {
    let mut file = File::open(path).expect(&("Failed to open ".to_owned() + path));
    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read vertex shader");

    CString::new(source.as_bytes()).unwrap()
}

fn ensure_compilation_success(shader_type: ShaderType, shader: u32) {
    unsafe {
        let max_len = 1024 as usize;
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(max_len);
        info_log.set_len(max_len - 1); // subtract 1 to skip the trailing null character
        match shader_type {
            ShaderType::Program => gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success),
            _ => gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success),
        }

        if success != gl::TRUE as GLint {
            let mut msg_len: i32 = -1;
            match shader_type {
                ShaderType::Program => gl::GetProgramInfoLog(shader, max_len as i32, &mut msg_len, info_log.as_mut_ptr() as *mut GLchar),
                _ => gl::GetShaderInfoLog(shader, max_len as i32, &mut msg_len, info_log.as_mut_ptr() as *mut GLchar),
            }
            info_log.truncate(msg_len as usize);
            panic!("ERROR::SHADER::{:?}::COMPILATION_FAILED\n{}", shader_type, str::from_utf8(&info_log).unwrap());
        }
    }
}
