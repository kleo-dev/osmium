pub mod threads;

use std::{any::Any, sync::Arc};

use osmium::{entity::Component, renderer::Point};

pub struct Jumping(pub u16, pub u16);
pub struct ZigZag(pub u16);

pub struct Velocity(pub Point<i32>);

impl Component for Velocity {
    fn tick(&mut self, entity: &std::sync::Arc<osmium::entity::Entity>, tick: u16) {
        let mut position = entity.position();
        apply_velocity(tick, self.0.x, &mut position.x);
        apply_velocity(tick, self.0.y, &mut position.y);
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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

impl Component for Jumping {
    fn tick(&mut self, entity: &Arc<osmium::entity::Entity>, _tick: u16) {
        let target = self.1;

        let mut pos = entity.position();

        // on first tick, start upward velocity
        if self.0 == 0 {
            std::thread::spawn({
                let entity = entity.clone();
                move || {
                    entity.update_component(|v: &mut Velocity| {
                        v.0.y = -150;
                    });
                }
            });
        }

        // If above the peak, fall down
        if pos.y < target - 6 {
            std::thread::spawn({
                let entity = entity.clone();
                move || {
                    entity.update_component(|v: &mut Velocity| {
                        v.0.y = 150;
                    });
                }
            });
        }

        // If we reached the ground, stop and remove component
        if self.0 > 30 && pos.y >= target {
            pos.y = target;
            std::thread::spawn({
                let entity = entity.clone();
                move || {
                    entity.update_component(|v: &mut Velocity| {
                        v.0.y = 0;
                    });
                    entity.remove_component::<Jumping>();
                }
            });
        }

        self.0 += 1;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Component for ZigZag {
    fn tick(&mut self, entity: &Arc<osmium::entity::Entity>, _tick: u16) {
        let pos = entity.get_position();

        if pos.x == 0 {
            std::thread::spawn({
                let entity = entity.clone();
                move || {
                    entity.update_component(|v: &mut Velocity| {
                        v.0.x = 200;
                    });
                }
            });
        }

        if pos.x >= self.0 {
            std::thread::spawn({
                let entity = entity.clone();
                move || {
                    entity.update_component(|v: &mut Velocity| {
                        v.0.x = -200;
                    });
                }
            });
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
