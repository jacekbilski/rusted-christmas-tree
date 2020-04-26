extern crate gl;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use cgmath::Vector3;

use self::gl::types::*;

pub const CAMERA_UBO_BINDING_POINT: u32 = 0;
pub const LIGHTS_UBO_BINDING_POINT: u32 = 1;

pub struct Shader {
    pub id: u32,
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
            let shader = Shader { id: shader_program };
            let vertex_shader = shader.add_vertex_shader(vertex_path);
            let fragment_shader = shader.add_fragment_shader(fragment_path);
            gl::LinkProgram(shader_program);
            ensure_compilation_success(ShaderType::Program, shader_program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            shader.bind_camera_ubo();
            shader.bind_lights_ubo();
            shader
        }
    }

    unsafe fn bind_camera_ubo(&self) {
        let c_name = CString::new("Camera").unwrap();
        let uniform_block_index = gl::GetUniformBlockIndex(self.id, c_name.as_ptr());
        gl::UniformBlockBinding(self.id, uniform_block_index, CAMERA_UBO_BINDING_POINT);
    }

    unsafe fn bind_lights_ubo(&self) {
        let c_name = CString::new("Lights").unwrap();
        let uniform_block_index = gl::GetUniformBlockIndex(self.id, c_name.as_ptr());
        gl::UniformBlockBinding(self.id, uniform_block_index, LIGHTS_UBO_BINDING_POINT);
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

    pub fn set_vector3(&self, name: &str, vec: Vector3<f32>) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform3f(location, vec[0], vec[1], vec[2]);
        }
    }

    pub fn set_float(&self, name: &str, f: f32) {
        unsafe {
            let c_name = CString::new(name).unwrap();
            let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
            gl::Uniform1f(location, f);
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
