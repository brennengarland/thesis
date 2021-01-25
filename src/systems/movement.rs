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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement() {
        pub struct Tester;
        impl<'a> System<'a> for Tester {
            type SystemData = (
                ReadStorage<'a, Position>,
                ReadStorage<'a, Velocity>,
                ReadStorage<'a, Antenna>
            );
            
            fn run(&mut self, (positions, velocities, antennas): Self::SystemData) {
                for (pos, _ant) in (&positions, &antennas).join() {
                    assert_eq!(pos , &Position{
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                        direction: 5.0
                    });
                }
                for (pos, _vel) in (&positions, &velocities).join() {
                    assert_eq!(pos , &Position{
                        x: -10.0,
                        y: 0.0,
                        z: 0.0,
                        direction: 0.0
                    });
                }
            }
            
        }

        // Create world
        let mut world = World::new();

        // Register the components to be used
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Antenna>();

        // Initialize systems
        let mut sys = Movement;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create radar entity
        let _radar: specs::Entity = world.create_entity()
        .with(Position{
            x: 0.0, 
            y: 0.0, 
            z: 0.0, 
            direction: 0.0
        }).with(Antenna{
            frequency: 100.0, 
            gain: 10.0_f32.powf(10.0 / 10.0), 
            power: (10.0 * 1000.0), 
            wavelength: ((3.0 * 100000000.0) / 100.0),
            azimuth_beam_width: 10.0,
            elevation_beam_width: 20.0
        }).build();
        // Create Target Entity
        let _target1 = world.create_entity()
        .with(Position{
            x: 0.0, 
            y: 0.0, 
            z: 0.0, 
            direction: 0.0
        }).with(Velocity{
            x: -10.0, 
            y: 0.0, 
            z: 0.0
        }).build();

        // Run the system
        sys.run_now(&world);
        world.maintain();
        // Run test 
        tester.run_now(&world);
    }
}