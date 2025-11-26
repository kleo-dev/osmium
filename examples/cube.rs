use osmium::entity::Component;

pub struct Velocity(pub i32);

impl Component for Velocity {
    fn tick(&mut self) {}
}

fn main() {
    let mut engine = osmium::init();

    engine.thread("Main.tick", || {});

    let mut my_rect = engine.draw(|r| {
        // r.draw_rect(0, 0, 10, 10);
    });

    my_rect.component(Velocity(10));
}
