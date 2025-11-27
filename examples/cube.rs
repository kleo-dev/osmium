pub mod components;

use osmium::{renderer::Point, terminal};

fn main() {
    let engine = osmium::init();

    engine.entity(|r| {
        r.draw_vertex(
            0,
            0,
            &[
                Point { x: 0, y: 0 },
                Point { x: 30, y: 0 },
                Point { x: 15, y: 8 },
            ],
            0x0000ff,
        );

        r.draw_vertex_outline(
            0,
            0,
            &[
                Point { x: 0, y: 0 },
                Point { x: 30, y: 0 },
                Point { x: 15, y: 8 },
            ],
            0xff0000,
        );
    });

    terminal::hide_cursor().unwrap();

    engine.start();

    terminal::show_cursor().unwrap();
}
