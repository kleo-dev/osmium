pub mod entity;
pub mod renderer;
pub mod terminal;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::renderer::Renderer;

pub fn init() -> Arc<Engine> {
    Arc::new(Engine {
        threads: Mutex::new(HashMap::new()),
        entities: Mutex::new(Vec::new()),
    })
}

pub struct Engine {
    threads: Mutex<HashMap<String, Arc<dyn Fn(&Arc<Self>) + Send + Sync>>>,
    entities: Mutex<Vec<Arc<entity::Entity>>>,
}

pub fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

impl Engine {
    pub fn thread<F: Fn(&Arc<Self>) + Send + Sync + 'static>(self: &Arc<Self>, label: &str, f: F) {
        self.threads
            .lock()
            .unwrap()
            .insert(label.to_string(), Arc::new(f));
    }

    pub fn entity<F: Fn(&mut Renderer) + Send + Sync + 'static>(
        self: &Arc<Self>,
        f: F,
    ) -> Arc<entity::Entity> {
        let e = entity::Entity::new(f);
        let mut entities = self.entities.lock().unwrap();
        entities.push(e);
        entities
            .last_mut()
            .expect("Unable to obtain entity (unspawned)")
            .clone()
    }

    pub fn render(self: &Arc<Self>) {
        terminal::clear().unwrap();

        let mut renderer = renderer::Renderer::new();
        for entity in self.entities.lock().unwrap().iter() {
            renderer.set_position_point(entity.get_position());
            entity.render(&mut renderer);
        }
    }

    pub fn tick(self: &Arc<Self>, tick: u16) {
        for entity in self.entities.lock().unwrap().iter() {
            entity.tick(tick);
        }
    }

    pub fn start(self: &Arc<Self>) {
        for thr in self.threads.lock().unwrap().values() {
            let thr = thr.clone();
            let engine = self.clone();
            std::thread::spawn(move || (thr)(&engine));
        }

        loop {
            self.render();
            sleep(16);
        }
    }
}
