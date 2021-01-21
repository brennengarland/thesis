use super::*;

pub struct AntennaReceiverSystem;
// Radar Sensor reads from environment
impl<'a> System<'a> for AntennaReceiverSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, EMWave>,
        ReadStorage<'a, Antenna>,
        Entities<'a>,
    );

    fn run(&mut self, (positions, emissions, antennas, entities) : Self::SystemData) {
        for (_antenna, antenna_pos) in (&antennas, &positions).join() {
            for(em_entity, em, em_pos) in (&*entities, &emissions, &positions).join() {
                let angle = incident_angle(em_pos, antenna_pos);
                if check_illumination(em.azimuth_width, em_pos.direction, angle) {
                    println!("Radar detected emission from angle: {}", antenna_pos.direction);
                    let range = calculate_range(&em_pos, &antenna_pos);
                    let _power = em.power / (4.0 * std::f32::consts::PI * range.powi(2));
                    let _time = range / (3.0 * (100000000.0));
                }
            
                match entities.delete(em_entity) {
                    Ok(r) => r,
                    Err(e) => eprintln!("Error!\n {}", e),
                }
            }
        }

    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_receiver() {
//         pub struct Tester;
//         impl<'a> System<'a> for Tester {
//             type SystemData = ReadStorage<'a, TargetIllumination>;
            
//             fn run(&mut self, illuminations: Self::SystemData) {
//                 for illums in (&illuminations).join() {
//                     // Make sure the system made the correct number of illuminations
//                     assert_eq!(illums.illuminations.len(), 1); 

//                     for illum in illums.illuminations.iter() {
//                         assert_eq!(illum, &Illumination{
//                             angle: 225.0,
//                             frequency: 100.0,
//                             lambda: 100.0,
//                             rcs: 180.0,
//                             power: 0.00039788734
//                         });
//                     }
//                 }
//             }
            
//         }

//         // Create world
//         let mut world = World::new();

//         // Register the components to be used
//         world.register::<EMWave>();
//         world.register::<Position>();
//         world.register::<Antenna>();

//         let mut sys = InteractionDetection;
//         System::setup(&mut sys, &mut world);

//         let mut tester = Tester;
//         System::setup(&mut tester, &mut world);

//         // Create EMWave Entity
//         let _em_wave = world.create_entity()
//         .with(EMWave{
//             frequency: 100.0,
//             power: 100.0,
//             wavelength: 100.0,
//             azimuth_width: 20.0,
//             elevation_width: 10.0
//         }).with(Position{
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//             direction: 45.0
//         }).build();

//         // Create target entity
//         let _target1 = world.create_entity()
//         .with(Position{
//             x: 100.0, 
//             y: 100.0, 
//             z: 0.0, 
//             direction: 0.0
//         }).with(RCS{
//             angles: vec![0.0, 90.0, 180.0, 270.0],
//             values: vec![0.0, 90.0, 180.0, 270.0],
//             avg_rcs: 180.0
//         }).with(TargetIllumination{
//             illuminations: Vec::new()
//         }).build();

//         // Run the system
//         sys.run_now(&world);
//         world.maintain();
//         // Run test 
//         tester.run_now(&world);
//     }
// }