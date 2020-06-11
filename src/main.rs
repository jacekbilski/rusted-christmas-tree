extern crate gl;
extern crate glfw;

use std::f32::consts::{FRAC_PI_8, PI};
use std::sync::mpsc::Receiver;

use fps_calculator::FpsCalculator;
use observer::RenderLoopObserver;
use xmas_tree::scene::Scene;

use self::glfw::{Action, Context, Glfw, Key, MouseButtonLeft, Window, WindowEvent};

mod camera;
mod coords;
mod model;
mod fps_calculator;
mod lights;
mod material;
mod observer;
mod shader;
mod xmas_tree;

// settings
const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;

struct Main {
    last_cursor_x: f64,
    last_cursor_y: f64,
}

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = setup_window(&mut glfw);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::MULTISAMPLE);
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        // gl::CullFace(gl::BACK);
        // gl::FrontFace(gl::CCW);
    }

    let mut scene = Scene::setup(&window);
    let mut fps_calculator = FpsCalculator::new();
    let mut main = Main { last_cursor_x: -1., last_cursor_y: -1. };

    // render loop
    while !window.should_close() {
        process_events(&mut main, &mut window, &events, &mut scene);
        scene.next_frame();
        scene.draw();
        window.swap_buffers();
        glfw.poll_events();
        fps_calculator.tick();
    }
}

fn setup_window(glfw: &mut Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));

    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "Rusted Christmas tree", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    (window, events)
}

fn process_events(main: &mut Main, window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, scene: &mut Scene) {
    let mut mouse_offset_x: f64 = 0.;
    let mut mouse_offset_y: f64 = 0.;
    for (_, event) in glfw::flush_messages(events) {
        let angle_change = FRAC_PI_8 / 4.;
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
                scene.camera.on_window_resize(&window);
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            glfw::WindowEvent::Key(Key::Right, _, Action::Press, _) | glfw::WindowEvent::Key(Key::Right, _, Action::Repeat, _) => {
                scene.camera.rotate_horizontally(angle_change);
            },
            glfw::WindowEvent::Key(Key::Left, _, Action::Press, _) | glfw::WindowEvent::Key(Key::Left, _, Action::Repeat, _) => {
                scene.camera.rotate_horizontally(-angle_change);
            },
            glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) | glfw::WindowEvent::Key(Key::Up, _, Action::Repeat, _) => {
                scene.camera.rotate_vertically(-angle_change);
            },
            glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) | glfw::WindowEvent::Key(Key::Down, _, Action::Repeat, _) => {
                scene.camera.rotate_vertically(angle_change);
            },
            glfw::WindowEvent::CursorPos(x, y) => {
                mouse_offset_x = x - main.last_cursor_x;
                mouse_offset_y = y - main.last_cursor_y;
                main.last_cursor_x = x;
                main.last_cursor_y = y;
            },
            _ => {}
        }
    }
    if window.get_mouse_button(MouseButtonLeft) == Action::Press && (mouse_offset_x != 0. || mouse_offset_y != 0.) {
        let (width, height) = window.get_size();
        let max = width.max(height) as f32;
        scene.camera.rotate_horizontally(-4. * PI / max * mouse_offset_x as f32);
        scene.camera.rotate_vertically(-4. * PI / max * mouse_offset_y as f32);
    }
}
