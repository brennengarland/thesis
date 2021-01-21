use super::*;

// Detects Interactions
pub struct InteractionDetection;
impl<'a> System<'a> for InteractionDetection {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, EMWave>,
        WriteStorage<'a, TargetIllumination>,
        ReadStorage<'a, RCS>,
        Entities<'a>,
    );

    fn run(&mut self, (positions, emissions, mut illumination, rcs, entities): Self::SystemData) {
        // Loop through all of the emissions. em_entity is just an identifier
        for (em_entity, em, em_pos) in (&*entities, &emissions, &positions).join() {
            // Loops through entities with only a position, illumination, and RCS. 
            // Should just be our 'targets'
            for(targ_rcs, targ_pos, ill) in (&rcs, &positions, &mut illumination).join() {
                let angle = incident_angle(em_pos, targ_pos);
                if check_illumination(em.azimuth_width, em_pos.direction, angle) {
                    let range = calculate_range(&em_pos, &targ_pos);
                    let power = em.power / (4.0 * std::f32::consts::PI * range.powi(2));
                    ill.illuminations.push(Illumination{
                        power: power, 
                        lambda: em.wavelength, 
                        frequency: em.frequency, 
                        angle: (angle + 180.0) % 360.0, // Change angle to target perspective 
                        rcs: targ_rcs.avg_rcs
                    });
                }
            }
            match entities.delete(em_entity) {
                Ok(r) => r,
                Err(e) => eprintln!("Error!\n {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction() {
        pub struct Tester;
        impl<'a> System<'a> for Tester {
            type SystemData = ReadStorage<'a, TargetIllumination>;
            
            fn run(&mut self, illuminations: Self::SystemData) {
                for illums in (&illuminations).join() {
                    // Make sure the system made the correct number of illuminations
                    assert_eq!(illums.illuminations.len(), 1); 

                    for illum in illums.illuminations.iter() {
                        assert_eq!(illum, &Illumination{
                            angle: 225.0,
                            frequency: 100.0,
                            lambda: 100.0,
                            rcs: 180.0,
                            power: 0.00039788734
                        });
                    }
                }
            }
            
        }

        // Create world
        let mut world = World::new();

        // Register the components to be used
        world.register::<EMWave>();
        world.register::<Position>();
        world.register::<Antenna>();

        let mut sys = InteractionDetection;
        System::setup(&mut sys, &mut world);

        let mut tester = Tester;
        System::setup(&mut tester, &mut world);

        // Create EMWave Entity
        let _em_wave = world.create_entity()
        .with(EMWave{
            frequency: 100.0,
            power: 100.0,
            wavelength: 100.0,
            azimuth_width: 20.0,
            elevation_width: 10.0
        }).with(Position{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            direction: 45.0
        }).build();

        // Create target entity
        let _target1 = world.create_entity()
        .with(Position{
            x: 100.0, 
            y: 100.0, 
            z: 0.0, 
            direction: 0.0
        }).with(RCS{
            angles: vec![0.0, 90.0, 180.0, 270.0],
            values: vec![0.0, 90.0, 180.0, 270.0],
            avg_rcs: 180.0
        }).with(TargetIllumination{
            illuminations: Vec::new()
        }).build();

        // Run the system
        sys.run_now(&world);
        world.maintain();
        // Run test 
        tester.run_now(&world);
    }
}

