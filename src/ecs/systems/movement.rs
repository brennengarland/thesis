use super::*;

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