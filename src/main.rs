extern crate gl;
extern crate glfw;

use std::boxed::Box;
use std::f32::consts::FRAC_PI_8;
use std::sync::mpsc::Receiver;

use cgmath::{Point3, vec3};

use drawable::Drawable;
use fps_calculator::FpsCalculator;
use observer::RenderLoopObserver;
use xmas_tree::XmasTree;

use crate::camera::Camera;
use crate::lights::Lights;

use self::glfw::{Action, Context, Glfw, Key, Window, WindowEvent};

mod camera;
mod coords;
mod drawable;
mod fps_calculator;
mod lights;
mod material;
mod observer;
mod shader;
mod xmas_tree;

// settings
const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;

struct Scene {
    camera: Camera,
    lights: Lights,
    obj: Box<dyn Drawable>,
}

impl Scene {
    fn setup(window: &Window) -> Self {
        let camera = Camera::new(Point3::new(15., 12., 12.), Point3::new(0., 0., 0.), &window);
        let mut lights = Lights::setup();
        lights.add(Point3::new(10., 100., 10.), vec3(0.3, 0.3, 0.3), vec3(0.2, 0.2, 0.2), vec3(0., 0., 0.));
        lights.add(Point3::new(5., 6., 2.), vec3(0.2, 0.2, 0.2), vec3(2., 2., 2.), vec3(0.5, 0.5, 0.5));

        let obj: Box<dyn Drawable> = Box::new(XmasTree::setup());
        Scene { camera, lights, obj }
    }

    pub fn draw(&mut self) {
        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            self.obj.draw();
        }
    }
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
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);
    }

    let mut scene = Scene::setup(&window);
    let mut fps_calculator = FpsCalculator::new();

    // render loop
    while !window.should_close() {
        process_events(&mut window, &events, &mut scene);
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
    (window, events)
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, scene: &mut Scene) {
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
            _ => {}
        }
    }
}
