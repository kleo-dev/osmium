use osmium::{entity::Component, sleep, terminal};

fn main() {
    terminal::hide_cursor().unwrap();

    let engine = osmium::init();

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

    let my_rect = engine.entity(|r| {
        r.draw_rect(0, 0, 1, 2, 0xff0000);
        r.draw_rect(1, 0, 1, 2, 0x00ff00);
        r.draw_rect(2, 0, 1, 2, 0x0000ff);
    });

    my_rect.component(Velocity(300));

    engine.start();

    terminal::show_cursor().unwrap();
}

pub struct Velocity(pub i32);

impl Component for Velocity {
    fn tick(&mut self, entity: &std::sync::Arc<osmium::entity::Entity>, tick: u16) {
        let mut position = entity.position();
        apply_velocity(tick, self.0, &mut position.x);
    }
}

fn apply_velocity(tick: u16, velocity: i32, x: &mut u16) -> bool {
    let abs_v = velocity.abs();
    if abs_v == 0 {
        return false;
    }

    let interval = (1000 / abs_v).max(1);
    if tick as i32 % interval == 0 {
        if velocity > 0 {
            *x = x.saturating_add(1);
        } else {
            *x = x.saturating_sub(1);
        }
        return true;
    }
    false
}
