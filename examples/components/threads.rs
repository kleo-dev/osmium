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
    engine.thread("Main.key", |engine| {
        loop {
            // Pass an event to the thread
            
        }
    });
}
