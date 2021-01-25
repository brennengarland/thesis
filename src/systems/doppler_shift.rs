use super::*;

pub struct DopplerShiftSystem;
impl<'a> System<'a> for DopplerShiftSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, TargetIllumination>,
    );

    fn run(&mut self, (velocities, mut target_ills) : Self::SystemData) {
        for (vel, targ) in (&velocities, &mut target_ills).join() {
            for ill in targ.illuminations.iter_mut() {
                ill.frequency = doppler_shift(vel, ill);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doppler() {
        pub struct Tester;
        impl<'a> System<'a> for Tester {
            type SystemData = ReadStorage<'a, TargetIllumination>;
            
            fn run(&mut self, illuminations: Self::SystemData) {
                for illums in (&illuminations).join() {
                    // Make sure the system made the correct number of illuminations
                    assert_eq!(illums.illuminations.len(), 1); 
                    for illum in illums.illuminations.iter() {
                        assert_eq!(illum, &Illumination{
                            angle: 90.0,
                            frequency: 10.00001,
                            lambda: 100.0,
                            rcs: 1.0,
                            power: 10.0
                        });
                    }
                }
            }
            
        }

        // Create world
        let mut world = World::new();

        // Register the components to be used
        world.register::<Velocity>();
        world.register::<TargetIllumination>();

        // Initialize systems
        let mut sys = DopplerShiftSystem;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create illumination entity
        let _target_illum = world.create_entity()
        .with(Velocity{
            x: 100.0,
            y: 100.0,
            z: 0.0
        }).with(TargetIllumination{
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