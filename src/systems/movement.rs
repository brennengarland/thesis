use specs::prelude::*;
use crate::components::{Position, Velocity, Antenna};

fn move_entity(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x;
    pos.y += vel.y;
}

fn rotate_entity(pos: &mut Position, sen: &Antenna) {
    pos.direction = (pos.direction + sen.azimuth_beam_width / 2.0) % 360.0;

}

// Changes the position of each entity with position and velocity
pub struct Movement;
impl<'a> System<'a> for Movement {

    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Antenna>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut position, mut sensor, velocity): Self::SystemData) {
        for(pos, sen) in (&mut position, &mut sensor).join() {
            rotate_entity(pos, sen);
        }

        for(pos, vel) in (&mut position, &velocity).join() {
            move_entity(pos, vel);
        }
    }
}