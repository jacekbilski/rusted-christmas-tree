use crate::drawable::Drawable;
use crate::xmas_tree::snow::Snow;
use crate::xmas_tree::static_object::StaticObject;

mod static_object;
mod baubles;
mod ground;
mod snow;
mod tree;

pub struct XmasTree {
    drawables: Vec<Box<dyn Drawable>>,
}

impl XmasTree {
    pub fn setup() -> Self {
        let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
        let ground = ground::gen_objects();
        drawables.push(Box::new(StaticObject::new(ground.0, ground.1, ground.2)));
        let tree = tree::gen_objects();
        drawables.push(Box::new(StaticObject::new(tree.0, tree.1, tree.2)));
        let baubles = baubles::gen_objects();
        drawables.push(Box::new(StaticObject::new(baubles.0, baubles.1, baubles.2)));
        drawables.push(Box::new(Snow::new()));
        XmasTree { drawables }
    }
}

impl Drawable for XmasTree {
    fn draw(&mut self) {
        for d in &mut self.drawables {
            d.draw();
        }
    }
}
