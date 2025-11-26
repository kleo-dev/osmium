use osmium::terminal;

fn main() {
    terminal::hide_cursor().unwrap();

    let mut engine = osmium::init();

    engine.thread("Main.tick", || {});

    let my_rect = engine.entity(|r| {
        r.draw_rect(8, 3, 11, 5, 0x0000ff);
    });

    terminal::clear().unwrap();

    my_rect.render(&mut osmium::renderer::Renderer {});

    terminal::show_cursor().unwrap();
}
