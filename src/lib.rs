pub mod entity;

use std::collections::HashMap;

pub fn init() -> Engine {
    Engine {
        threads: HashMap::new(),
        entities: Vec::new(),
    }
}

pub struct Engine {
    threads: HashMap<String, Box<dyn FnMut() + Send + Sync>>,
    entities: Vec<entity::Entity>,
}

pub struct Renderer {}

impl Engine {
    pub fn thread<F: FnMut() + Send + Sync + 'static>(&mut self, label: &str, f: F) {
        self.threads.insert(label.to_string(), Box::new(f));
    }

    pub fn draw<F: FnMut(&mut Renderer) + 'static>(&self, mut f: F) -> entity::Entity {
        let mut renderer = Renderer {};
        (f)(&mut renderer);
        entity::Entity(Vec::new())
    }
}
