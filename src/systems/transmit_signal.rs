use super::*;

pub struct TransmitSignal;
impl<'a> System<'a> for TransmitSignal {
    type SystemData = (
        ReadStorage<'a, Antenna>,
        WriteStorage<'a, Position>,
        Entities<'a>,
        Read<'a, LazyUpdate>
    );

    fn run(&mut self, 
        (antennas,  mut positions, entities, updater): Self::SystemData) {

        // Must Read from each radar system and save values, 
        // then create the new emission afterwards
        // because we cannot iterate over positions and write to them at the same time.
        for (ant, pos) in (&antennas, &mut positions).join() {
            let new_pos = Position{
                x: pos.x, 
                y: pos.y, 
                z: pos.z, 
                direction: pos.direction
            };
            let new_wave = EMWave{
                power: (ant.power*ant.gain), 
                wavelength: ant.wavelength, 
                frequency: ant.frequency, 
                azimuth_width: ant.azimuth_beam_width, 
                elevation_width: ant.elevation_beam_width
            };
            let new_entity = entities.create();
            updater.insert(new_entity, new_pos);
            updater.insert(new_entity, new_wave);
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
                assert_eq!(em_waves.count(), 1);
                for em_wave in (&em_waves).join() {
                    assert_eq!(em_wave , &EMWave{
                        power: 10000.0, 
                        wavelength: 1000.0, 
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

        let mut sys = TransmitSignal;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create radar entity
        let _radar = world.create_entity()
        .with(Position{
            x: 0.0, 
            y: 0.0, 
            z: 0.0, 
            direction: 5.0
        })
        .with(Antenna{
            frequency: 100.0, 
            gain: 10.0, 
            power: 1000.0, 
            wavelength: 1000.0,
            azimuth_beam_width: 10.0,
            elevation_beam_width: 20.0
        }).build();

        // Run the system
        sys.run_now(&world);
        world.maintain();
        // Run test 
        tester.run_now(&world);
    }
}