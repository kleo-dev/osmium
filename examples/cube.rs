pub mod components;

use osmium::{renderer::Point, terminal};

fn main() {
    let engine = osmium::init();

    let my_rect = engine.entity(|r| {
        r.draw_rect(0, 0, 1, 2, 0xff0000);
        r.draw_rect(1, 0, 1, 2, 0x00ff00);
        r.draw_rect(2, 0, 1, 2, 0x0000ff);
    });

    my_rect.component(components::Velocity(Point { x: 0, y: 0 }));

    components::threads::tick(&engine);
    components::threads::key(&engine);

    engine.on_event({
        let my_rect = my_rect.clone();
        move |e: &crossterm::event::Event| {
            if let Some(key_event) = e.as_key_event() {
                match key_event.code {
                    crossterm::event::KeyCode::Right => {
                        my_rect.update_component(|c: &mut components::Velocity| {
                            c.0.x = 100;
                        });
                    }
                    crossterm::event::KeyCode::Left => {
                        my_rect.update_component(|c: &mut components::Velocity| {
                            c.0.x = -100;
                        });
                    }

                    crossterm::event::KeyCode::Down => {
                        my_rect.update_component(|c: &mut components::Velocity| {
                            c.0.y = 100;
                        });
                    }
                    crossterm::event::KeyCode::Up => {
                        my_rect.update_component(|c: &mut components::Velocity| {
                            c.0.y = -100;
                        });
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
