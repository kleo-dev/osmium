use std::sync::Arc;

use osmium::{Engine, sleep};

pub fn tick(engine: &Arc<Engine>) {
    engine.thread("Main.tick", |engine| {
        let mut tick = 0;
        loop {
            if tick == 60000 {
                tick = 0;
            }

            engine.tick(tick);

            tick += 1;
            sleep(4);
        }
    });
}

pub fn key(engine: &Arc<Engine>) {
    crossterm::terminal::enable_raw_mode().unwrap();

    engine.thread("Main.key", |engine| {
        loop {
            engine.emit_event(crossterm::event::read().unwrap());
        }
    });
}
