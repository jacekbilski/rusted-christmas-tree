extern crate gl;

use std::ffi::CString;
use std::ptr;
use std::str;

use cgmath::{Matrix4, Vector3};
use cgmath::prelude::*;

use self::gl::types::*;

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
    pub fn new() -> Shader {
        let vertex_shader = Shader::setup_vertex_shader();
        let fragment_shader = Shader::setup_fragment_shader();

        unsafe {
            // link shaders
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            Shader::ensure_compilation_success(ShaderType::Program, shader_program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            Shader { id: shader_program }
        }
    }

    fn setup_vertex_shader() -> u32 {
        const VERTEX_SHADER_SOURCE: &str = r#"
        #version 330 core

        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aCol;
        layout (location = 2) in vec3 aNormal;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        out vec3 FragPosition;
        out vec3 Colour;
        out vec3 Normal;

        void main() {
            gl_Position = projection * view * model * vec4(aPos, 1.0);
            FragPosition = vec3(model * vec4(aPos, 1.0));
            Colour = aCol;
            Normal = aNormal;
        }
    "#;

        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            Shader::ensure_compilation_success(ShaderType::VertexShader, vertex_shader);
            vertex_shader
        }
    }

    fn setup_fragment_shader() -> u32 {
        const FRAGMENT_SHADER_SOURCE: &str = r#"
        #version 330 core

        in vec3 FragPosition;
        in vec3 Colour;
        in vec3 Normal;

        uniform vec3 lightColour;
        uniform vec3 lightPosition;

        out vec4 FragColor;

        const float ambientStrength = 0.1;

        void main() {
            vec3 ambient = ambientStrength * lightColour;

            vec3 norm = normalize(Normal);
            vec3 lightDir = normalize(lightPosition - FragPosition);
            float diff = max(dot(norm, lightDir), 0.0);
            vec3 diffuse = diff * lightColour;

            FragColor = vec4((ambient + diffuse) * Colour, 1.0);
        }
    "#;

        unsafe {
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            Shader::ensure_compilation_success(ShaderType::FragmentShader, fragment_shader);
            fragment_shader
        }
    }

    fn ensure_compilation_success(shader_type: ShaderType, shader: u32) {
        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            match shader_type {
                ShaderType::Program => gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success),
                _ => gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success),
            }

            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                // fishy - doesn't work
                println!("ERROR::SHADER::{:?}::COMPILATION_FAILED\n{}", shader_type, str::from_utf8(&info_log).unwrap());
                // panic ?
            }
        }
    }

    pub unsafe fn set_vec3(&self, name: &str, vec: &Vector3<f32>) {
        let c_name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
        gl::Uniform3fv(location, 1, vec.as_ptr());
    }

    pub unsafe fn set_mat4(&self, name: &str, mat: &Matrix4<f32>) {
        let c_name = CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, c_name.as_ptr());
        gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
    }
}
