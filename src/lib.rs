use std::collections::HashMap;

pub fn init() -> Engine {
    Engine {
        threads: HashMap::new(),
    }
}

pub struct Engine {
    threads: HashMap<String, Box<dyn FnMut() + Send + Sync>>,
}

impl Engine {
    pub fn thread<F: FnMut() + Send + Sync + 'static>(&mut self, label: &str, f: F) {
        self.threads.insert(label.to_string(), Box::new(f));
    }
}
