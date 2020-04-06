extern crate gl;
extern crate glfw;

use std::collections::VecDeque;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;
use std::sync::mpsc::Receiver;
use std::time::Instant;

use self::gl::types::*;
use self::glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

// settings
const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;
const FPS_ARRAY_SIZE: usize = 100;

#[derive(Debug)]
enum ShaderType {
    VertexShader,
    FragmentShader,
    Program
}

type VBO = u32;
type VAO = u32;
type EBO = u32;

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = setup_window(&mut glfw);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_program, vao) = setup_drawing_triangle();

    let mut frame_times: VecDeque<Instant> = VecDeque::with_capacity(FPS_ARRAY_SIZE);
    frame_times.push_back(Instant::now());

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);
        render(&mut glfw, shader_program, vao);
        window.swap_buffers();
        glfw.poll_events();
        calc_and_print_fps(&mut frame_times);
    }
}

fn setup_window(glfw: &mut Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    (window, events)
}

fn setup_drawing_triangle() -> (u32, u32) {
    let shader_program = setup_shader_program();

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
    let vertices: [f32; 12] = [
        -0.8, 0.0, 0.0, // left
        0.0, 0.7, 0.0,  // top
        0.0, -0.7, 0.0, // bottom
        0.8, 0.0, 0.0, // right
    ];
    let indices: [u32; 6] = [
        0, 1, 2,
        3, 1, 2,
    ];

    let within_vao = || {
        create_vbo(&vertices);
        create_ebo(&indices);
    };

    let vao = create_vao(within_vao);

    (shader_program, vao)
}

fn create_vao(within_vao_context: impl Fn() -> ()) -> VAO {
    unsafe {
        let mut vao = 0 as VAO;
        gl::GenVertexArrays(1, &mut vao); // create VAO
        gl::BindVertexArray(vao); // ...and bind it

        within_vao_context();

        // tell GL how to interpret the data in VBO -> one triangle vertex takes 3 coordinates (x, y, z)
        // this call also connects my VBO to this attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0); // enable the attribute

        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind my VBO
        // do NOT unbind EBO, VAO would remember that
        gl::BindVertexArray(0); // unbind my VAO
        vao
    }
}

fn create_vbo(vertices: &[f32; 12]) {
    unsafe {
        let mut vbo = 0 as VBO;
        gl::GenBuffers(1, &mut vbo); // create buffer for my data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // ARRAY_BUFFER now "points" to my buffer
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW); // actually fill ARRAY_BUFFER (my buffer) with data
    }
}

fn create_ebo(indices: &[u32; 6]) {
    unsafe {
        let mut ebo = 0 as EBO;
        gl::GenBuffers(1, &mut ebo); // create buffer for indices (elements)
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo); // ELEMENT_ARRAY_BUFFER now "points" to my buffer
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                       &indices[0] as *const u32 as *const c_void,
                       gl::STATIC_DRAW); // actually fill ELEMENT_ARRAY_BUFFER with data
    }
}

fn setup_shader_program() -> u32 {
    let vertex_shader = setup_vertex_shader();
    let fragment_shader = setup_fragment_shader();

    unsafe {
        // link shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        ensure_compilation_success(ShaderType::Program, shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        shader_program
    }
}

fn setup_vertex_shader() -> u32 {
    const VERTEX_SHADER_SOURCE: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        void main() {
           gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        ensure_compilation_success(ShaderType::VertexShader, vertex_shader);
        vertex_shader
    }
}

fn setup_fragment_shader() -> u32 {
    const FRAGMENT_SHADER_SOURCE: &str = r#"
        #version 330 core
        uniform vec4 myColor;
        out vec4 FragColor;
        void main() {
           FragColor = myColor;
        }
    "#;

    unsafe {
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        ensure_compilation_success(ShaderType::FragmentShader, fragment_shader);
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

fn render(glfw: &mut Glfw, shader_program: u32, vao: u32) {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        draw_triangle(glfw, shader_program, vao);
    }
}

fn draw_triangle(glfw: &mut Glfw, shader_program: u32, vao: u32) {
    unsafe {
        gl::UseProgram(shader_program);

        let time_value = glfw.get_time() as f32;
        let green_value = time_value.sin() / 2.0 + 0.5;
        let my_color = CString::new("myColor").unwrap();
        let vertex_color_location = gl::GetUniformLocation(shader_program, my_color.as_ptr());
        gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

        gl::BindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        // gl::BindVertexArray(0); // no need to unbind it every time
    }
}

fn calc_and_print_fps(frame_times: &mut VecDeque<Instant>) {
    let earliest_frame = if frame_times.len() == FPS_ARRAY_SIZE {
        frame_times.pop_front().unwrap()
    } else {
        *(frame_times.front().unwrap())
    };
    let elapsed = earliest_frame.elapsed();
    let fps = 1000000.0 * frame_times.len() as f64 / elapsed.as_micros() as f64;
    println!("FPS: {:?}, elapsed: {:?}", fps, elapsed);
    frame_times.push_back(Instant::now());
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
