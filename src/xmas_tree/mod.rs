use glfw::Window;

use ground::Ground;
use tree::Tree;

use crate::drawable::Drawable;

mod ground;
mod tree;

pub struct XmasTree {
    drawables: Vec<Box<dyn Drawable>>,
}

impl XmasTree {
    pub fn setup() -> Self {
        let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
        drawables.push(Box::new(Ground::setup()));
        drawables.push(Box::new(Tree::setup()));
        XmasTree { drawables }
    }
}

impl Drawable for XmasTree {
    fn draw(&self, window: &mut Window) {
        for d in &self.drawables {
            d.draw(window);
        }
    }
}
