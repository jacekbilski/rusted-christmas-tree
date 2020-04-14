use glfw::Window;

pub trait Drawable {
    fn draw(&self, window: &mut Window);
}
