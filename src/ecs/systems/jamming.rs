use super::*;

pub struct JammingSystem;
impl<'a> System<'a> for JammingSystem {
    type SystemData = (
        ReadStorage<'a, TargetIllumination>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        ReadStorage <'a, Antenna>,
        Entities<'a>,
    );

    fn run(&mut self, (target_illumination, mut emission, mut position, antenna, entities) : Self::SystemData) {
        
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<EMWave> = Vec::new();
        // Iterate through each target
        for (target, pos, ant) in (&target_illumination, &mut position, &antenna).join() {
            for _ill in target.illuminations.iter() {
                let position = Position{
                    x: pos.x, 
                    y: pos.y, 
                    z: pos.z, 
                    direction: pos.direction
                };
                let emission = EMWave{
                    power: (ant.power*ant.gain), 
                    wavelength: ant.wavelength, 
                    frequency: ant.frequency, 
                    azimuth_width: ant.azimuth_beam_width, 
                    elevation_width: ant.elevation_beam_width
                };
                    
                new_positions.push(position);
                new_emissions.push(emission);
            }
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
    fn test_transmit() {
        pub struct Tester;
        impl<'a> System<'a> for Tester {
            type SystemData = ReadStorage<'a, EMWave>;
            
            fn run(&mut self, em_waves: Self::SystemData) {
                for em_wave in (&em_waves).join() {
                    assert_eq!(em_wave , &EMWave{
                        power: 100.0, 
                        wavelength: ((3.0 * 100000000.0) / 100.0), 
                        frequency: 100.0, 
                        azimuth_width: 10.0, 
                        elevation_width: 20.0
                    });
                }
            }
            
        }

        // Create world
        let mut world = World::new();

        // Register the components to be used
        world.register::<EMWave>();
        world.register::<Position>();
        world.register::<Antenna>();

        let mut sys = JammingSystem;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create illumination entity
        let _target_illum = world.create_entity()
        .with(Antenna{
            frequency: 100.0, 
            gain: 10.0, 
            power: 10.0, 
            wavelength: ((3.0 * 100000000.0) / 100.0),
            azimuth_beam_width: 10.0,
            elevation_beam_width: 20.0
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