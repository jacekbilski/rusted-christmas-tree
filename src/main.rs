extern crate gl;
extern crate glfw;

use std::boxed::Box;
use std::sync::mpsc::Receiver;

use drawable::Drawable;
use fps_calculator::FpsCalculator;
use observer::RenderLoopObserver;
use xmas_tree::Ground;

use self::glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

mod drawable;
mod fps_calculator;
mod observer;
mod shader;
mod xmas_tree;

// settings
const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;

fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = setup_window(&mut glfw);

    // gl: load all OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl::Enable(gl::MULTISAMPLE);
    }

    let obj: Box<dyn Drawable> = Box::new(Ground::setup());
    let mut observer = FpsCalculator::new();

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events);
        render(&mut window, &obj);
        window.swap_buffers();
        glfw.poll_events();
        observer.tick();
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
    (window, events)
}

fn render(window: &mut Window, obj: &Box<dyn Drawable>) {
    unsafe {
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        obj.draw(window);
    }
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
