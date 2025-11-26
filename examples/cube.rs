use osmium::entity::Component;

fn main() {
    let mut engine = osmium::init();

    engine.thread("Main.tick", || {});

    let mut my_rect = engine.entity(|r| {
        // r.draw_rect(0, 0, 10, 10);
    });
}
