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
            // Loops through entities with only a position, illumination, and RCS. Should just be our 'targets'
            for(targ_rcs, targ_pos, ill) in (&rcs, &positions, &mut illumination).join() {
                let angle = incident_angle(em_pos, targ_pos);
                if check_illumination(em.azimuth_width, em_pos.direction, angle) {
                    // Power received: Pr = (Pt * G^2 * lambda^2 * rcs) / ((4pi)^3 * R^4)
                    println!("!!!!Target Hit!!!!");
                    println!("Target Angle: {} at {}", angle, targ_pos.x);
                    // println!("Emission Direction: {}\tWidth: {}", em_dir, em_width);
                    let range = ((em_pos.x - targ_pos.x).powi(2) + (em_pos.y - targ_pos.y).powi(2)).sqrt();
                    let power = em.power / (4.0 * std::f32::consts::PI * range.powi(2));
                    let new_abs = Illumination{power: power, lambda: em.wavelength, frequency: em.frequency, angle: angle, rcs: targ_rcs.avg_rcs};
                    ill.illuminations.push(new_abs);
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
                    for illum in illums.illuminations.iter() {
                        assert_eq!(illum, &Illumination{
                            angle: 225.0,
                            frequency: 100.0,
                            lambda: 100.0,
                            rcs: 0.0,
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

