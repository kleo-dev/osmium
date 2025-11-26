use osmium::terminal;

fn main() {
    terminal::hide_cursor().unwrap();

    let mut engine = osmium::init();

    engine.thread("Main.tick", || {});

    let my_rect = engine.entity(|r| {
        r.draw_rect(0, 0, 2, 1, 0x0000ff);
    });

    engine.start();

    terminal::show_cursor().unwrap();
}
