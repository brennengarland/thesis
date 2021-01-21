use super::*;

pub struct RCSSystem;
impl<'a> System<'a> for RCSSystem {
    type SystemData = (
        ReadStorage<'a, RCS>,
        WriteStorage<'a, TargetIllumination>,
    );

    fn run(&mut self, (cross_sections, mut illuminations) : Self::SystemData)  {
        for (rcs, targ) in (&cross_sections, &mut illuminations).join() {
            for ill in targ.illuminations.iter_mut() {
                ill.rcs = calculate_rcs(ill.angle, &rcs.angles, &rcs.values);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcs() {
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
                            frequency: 10.0,
                            lambda: 100.0,
                            rcs: 90.0,
                            power: 10.0
                        });
                    }
                }
            }
            
        }

        // Create world
        let mut world = World::new();

        // Register the components to be used
        world.register::<RCS>();
        world.register::<TargetIllumination>();

        // Initialize systems
        let mut sys = RCSSystem;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create illumination entity
        let _target_illum = world.create_entity()
        .with(RCS{
            angles: vec![0.0, 90.0, 180.0, 270.0],
            values: vec![0.0, 90.0, 180.0, 270.0],
            avg_rcs: 180.0
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