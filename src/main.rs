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
use self::glfw::{Action, Context, Key};

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

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    // glfw window creation
    // --------------------
    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "LearnOpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_program, vao) = setup_drawing_triangle();

    let mut frame_times: VecDeque<Instant> = VecDeque::with_capacity(FPS_ARRAY_SIZE);
    frame_times.push_back(Instant::now());

    // render loop
    // -----------
    while !window.should_close() {

        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            draw_triangle(shader_program, vao);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();

        calc_and_print_fps(&mut frame_times);
    }
}

fn setup_drawing_triangle() -> (u32, u32) {
    unsafe {
        let shader_program = setup_shader_program();

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // left
            0.5, -0.5, 0.0, // right
            0.0, 0.5, 0.0  // top
        ];
        let (mut vbo, mut vao) = (0, 0 as u32);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (shader_program, vao)
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
        out vec4 FragColor;
        void main() {
           FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
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

fn draw_triangle(shader_program: u32, vao: u32) {
    unsafe {
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
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

// NOTE: not the same version as in common.rs!
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
