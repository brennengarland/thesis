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
                    let power = em.power / (4.0 * 3.14 * range.powi(2));
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