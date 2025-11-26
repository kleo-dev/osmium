pub mod components;

use osmium::{renderer::Point, terminal};

fn main() {
    let (term_width, term_height) = crossterm::terminal::size().unwrap();
    let a = term_height - 3;

    let engine = osmium::init();

    let my_rect = engine.entity(|r| {
        r.draw_rect(0, 0, 1, 2, 0xff0000);
        r.draw_rect(1, 0, 1, 2, 0x00ff00);
        r.draw_rect(2, 0, 1, 2, 0x0000ff);
    });

    my_rect.component(components::Velocity(Point { x: 0, y: 0 }));
    my_rect.component(components::ZigZag(term_width - 3));

    my_rect.set_position(0, term_height - 3);

    components::threads::tick(&engine);
    components::threads::key(&engine);

    engine.on_event({
        let my_rect = my_rect.clone();
        move |e: &crossterm::event::Event| {
            if let Some(key_event) = e.as_key_event() {
                match key_event.code {
                    crossterm::event::KeyCode::Char(' ') => {
                        my_rect.component(components::Jumping(0, a));
                    }
                    _ => {}
                }
            }
        }
    });

    terminal::hide_cursor().unwrap();

    engine.start();

    terminal::show_cursor().unwrap();
}
