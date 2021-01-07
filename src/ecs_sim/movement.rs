

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
            let max_direction = 360.0;  // degrees
            pos.direction = (pos.direction + sen.azimuth_beam_width/2.0) % max_direction;
        }

        for(pos, vel) in (&mut position, &velocity).join() {
            println!("x: {}, y: {}, z: {}", vel.x, vel.y, vel.z);
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}