use super::*;

pub fn move_entity(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x;
    pos.y += vel.y;
}