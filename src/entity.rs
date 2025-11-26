pub trait Component {
    fn init_add(&mut self) {}
    fn init(&mut self) {}
    fn tick(&mut self) {}
}

pub struct Entity(pub Vec<Box<dyn Component>>);

impl Entity {
    pub fn component<C: Component + 'static>(&mut self, c: C) {
        self.0.push(Box::new(c));
    }
}
