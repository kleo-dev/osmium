pub mod threads;

use osmium::{entity::Component, renderer::Point};

pub struct Velocity(pub Point<i32>);

impl Component for Velocity {
    fn tick(&mut self, entity: &std::sync::Arc<osmium::entity::Entity>, tick: u16) {
        let mut position = entity.position();
        apply_velocity(tick, self.0.x, &mut position.x);
        apply_velocity(tick, self.0.y, &mut position.y);
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
