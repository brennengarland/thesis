use super::*;

// Creates an emission from the absorption information
pub struct ReflectionSystem;
impl<'a> System<'a> for ReflectionSystem {
    type SystemData = (
        WriteStorage<'a, TargetIllumination>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        Entities<'a>,
    );

    fn run(&mut self, (mut target_illumination, mut emission, mut position, entities) : Self::SystemData) {
        
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<EMWave> = Vec::new();
        // Iterate through each target
        for (target, pos) in (&mut target_illumination, &position).join() {
            for ill in target.illuminations.iter() {
                let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: ill.angle};
                let p_r = ill.power * ill.rcs;
                let emission = EMWave{power: p_r, wavelength: ill.lambda, frequency: ill.frequency, azimuth_width: 20.0, elevation_width: 20.0};
                // println!("Emission Direction: {}", position.direction);
                new_positions.push(position);
                new_emissions.push(emission);
            }
            target.illuminations.clear();
        }

        while new_positions.len() != 0 {
            let new_entity = entities.create();
            match position.insert(new_entity, new_positions.remove(0)) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
            match emission.insert(new_entity, new_emissions.remove(0)) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection() {
        pub struct Tester;
        impl<'a> System<'a> for Tester {
            type SystemData = (
                ReadStorage<'a, EMWave>,
                ReadStorage<'a, Position>
            );
            
            fn run(&mut self, (em_waves, positions): Self::SystemData) {
                assert_eq!(em_waves.count(), 1);
                assert_eq!(positions.count(), 2);
                for (em_wave, pos) in (&em_waves, &positions).join() {
                    assert_eq!(em_wave , &EMWave{
                        power: 10.0, 
                        wavelength: 100.0, 
                        frequency: 10.0, 
                        azimuth_width: 20.0, 
                        elevation_width: 20.0
                    });
                    assert_eq!(pos, &Position{
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                        direction: 90.0
                    });
                }
            }
            
        }

        // Create world
        let mut world = World::new();

        // Register the components to be used
        world.register::<EMWave>();
        world.register::<Position>();
        world.register::<TargetIllumination>();

        // Initialize systems
        let mut sys = ReflectionSystem;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create illumination entity
        let _target_illum = world.create_entity()
        .with(Position{
            x: 0.0, 
            y: 0.0, 
            z: 1.0, 
            direction: 5.0
        })
        .with(TargetIllumination{
            illuminations: vec![Illumination{
                angle: 90.0,
                frequency: 10.0,
                lambda: 100.0,
                rcs: 1.0,
                power: 10.0
            }]
        }).build();

        // Run the system
        sys.run_now(&world);
        world.maintain();
        // Run test 
        tester.run_now(&world);
    }
}