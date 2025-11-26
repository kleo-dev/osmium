use crate::Renderer;

pub trait Component {
    #[allow(unused_variables)]
    fn init_add(&mut self, entity: &mut Entity) {}
    #[allow(unused_variables)]
    fn init(&mut self, entity: &mut Entity) {}
    #[allow(unused_variables)]
    fn tick(&mut self, entity: &mut Entity) {}
}

pub struct Entity {
    components: Vec<Box<dyn Component>>,
    render: Box<dyn FnMut(&mut Renderer)>,
}

impl Entity {
    pub fn new<F: FnMut(&mut Renderer) + 'static>(render: F) -> Self {
        Self {
            components: Vec::new(),
            render: Box::new(render),
        }
    }

    pub fn component<C: Component + 'static>(&mut self, c: C) {
        self.components.push(Box::new(c));
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        (self.render)(renderer)
    }
}
