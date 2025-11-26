use std::sync::{Arc, Mutex};

use crate::Renderer;

pub trait Component: Send + Sync {
    #[allow(unused_variables)]
    fn init_add(&mut self, entity: &Arc<Entity>) {}
    #[allow(unused_variables)]
    fn init(&mut self, entity: &Arc<Entity>) {}
    #[allow(unused_variables)]
    fn tick(&mut self, entity: &Arc<Entity>) {}
}

pub struct Entity {
    components: Mutex<Vec<Box<dyn Component>>>,
    render: Box<dyn Fn(&mut Renderer) + Send + Sync>,
}

impl Entity {
    pub fn new<F: Fn(&mut Renderer) + Send + Sync + 'static>(render: F) -> Arc<Self> {
        Arc::new(Self {
            components: Mutex::new(Vec::new()),
            render: Box::new(render),
        })
    }

    pub fn component<C: Component + 'static>(&mut self, c: C) {
        self.components.lock().unwrap().push(Box::new(c));
    }

    pub fn render(self: &Arc<Self>, renderer: &mut Renderer) {
        (self.render)(renderer)
    }

    pub fn tick(self: &Arc<Self>) {
        for comp in self.components.lock().unwrap().iter_mut() {
            comp.tick(self);
        }
    }
}
