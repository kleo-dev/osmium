use std::sync::{Arc, Mutex, MutexGuard};

use crate::{Renderer, renderer::Point};

pub trait Component: Send + Sync {
    #[allow(unused_variables)]
    fn init_add(&mut self, entity: &Arc<Entity>) {}
    #[allow(unused_variables)]
    fn init(&mut self, entity: &Arc<Entity>) {}
    #[allow(unused_variables)]
    fn tick(&mut self, entity: &Arc<Entity>, tick: u16) {}
}

pub struct Entity {
    pos: Mutex<Point>,
    components: Mutex<Vec<Box<dyn Component>>>,
    render: Box<dyn Fn(&mut Renderer) + Send + Sync>,
}

impl Entity {
    pub fn new<F: Fn(&mut Renderer) + Send + Sync + 'static>(render: F) -> Arc<Self> {
        Arc::new(Self {
            pos: Mutex::new(Point { x: 0, y: 0 }),
            components: Mutex::new(Vec::new()),
            render: Box::new(render),
        })
    }

    pub fn component<C: Component + 'static>(self: &Arc<Self>, c: C) {
        self.components.lock().unwrap().push(Box::new(c));
    }

    pub fn render(self: &Arc<Self>, renderer: &mut Renderer) {
        (self.render)(renderer)
    }

    pub fn tick(self: &Arc<Self>, tick: u16) {
        for comp in self.components.lock().unwrap().iter_mut() {
            comp.tick(self, tick);
        }
    }

    pub fn position<'a>(self: &'a Arc<Self>) -> MutexGuard<'a, Point> {
        self.pos.lock().unwrap()
    }

    pub fn get_position<'a>(self: &'a Arc<Self>) -> Point {
        self.pos.lock().unwrap().clone()
    }

    pub fn set_position(&mut self, x: u16, y: u16) {
        let mut pos = self.pos.lock().unwrap();
        pos.x = x;
        pos.y = y;
    }

    pub fn set_position_point<'a>(self: &'a Arc<Self>, pos: Point) {
        *self.pos.lock().unwrap() = pos;
    }
}
