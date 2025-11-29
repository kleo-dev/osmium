pub mod components;

use std::sync::{Arc, Mutex};

use osmium::{renderer::Point, terminal};

fn main() {
    let engine = osmium::init();

    let last_frame = Arc::new(Mutex::new(std::time::Instant::now()));
    let peak: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
    let low: Arc<Mutex<u32>> = Arc::new(Mutex::new(1000));

    engine.entity(move |r| {
        let current_frame = std::time::Instant::now();
        let since = current_frame
            .duration_since(last_frame.lock().unwrap().clone())
            .as_millis();

        if since > 0 {
            let fps: u32 = 1000 / since as u32;

            r.draw_text(
                0,
                0,
                format!("{fps}\n{}\n{}", peak.lock().unwrap(), low.lock().unwrap()),
            );

            {
                let mut peak = peak.lock().unwrap();
                let mut low = low.lock().unwrap();

                if fps > *peak {
                    *peak = fps;
                }

                if fps < *low {
                    *low = fps;
                }
            }
        }

        *last_frame.lock().unwrap() = current_frame;
    });

    engine
        .entity(|r| {
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
        })
        .set_position(20, 0);

    terminal::hide_cursor().unwrap();

    engine.start();

    terminal::show_cursor().unwrap();
}
