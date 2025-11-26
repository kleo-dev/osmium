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

    pub fn entity<F: FnMut(&mut Renderer) + 'static>(&mut self, f: F) -> &mut entity::Entity {
        let e = entity::Entity::new(f);
        self.entities.push(e);
        self.entities
            .last_mut()
            .expect("Unable to obtain entity (unspawned)")
    }
}
