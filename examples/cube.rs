use osmium::{sleep, terminal};

fn main() {
    terminal::hide_cursor().unwrap();

    let engine = osmium::init();

    engine.thread("Main.tick", |engine| {
        loop {
            engine.tick();
            sleep(4);
        }
    });

    let my_rect = engine.entity(|r| {
        r.draw_rect(0, 0, 1, 2, 0xff0000);
        r.draw_rect(1, 0, 1, 2, 0x00ff00);
        r.draw_rect(2, 0, 1, 2, 0x0000ff);
    });

    engine.start();

    terminal::show_cursor().unwrap();
}
