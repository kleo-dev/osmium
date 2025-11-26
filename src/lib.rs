pub mod entity;
pub mod renderer;
pub mod terminal;

use std::collections::HashMap;

use crate::renderer::Renderer;

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

pub fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

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

    pub fn render(&mut self) {
        terminal::clear().unwrap();

        let mut renderer = renderer::Renderer::new();
        for entity in &mut self.entities {
            entity.render(&mut renderer);
        }
    }

    pub fn tick(&mut self) {
        terminal::clear().unwrap();

        for entity in &mut self.entities {
            entity.tick();
        }
    }

    pub fn start(&mut self) {
        for thr in &mut self.threads.values() {
            //     std::thread::spawn(thr);
        }

        loop {
            self.render();
            sleep(16);
        }
    }
}
