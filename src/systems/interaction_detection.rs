use specs::prelude::*;
use crate::components::{Position, EMWave, TargetIllumination, RCS, Illumination};

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
                let y = targ_pos.y - em_pos.y;
                let x = targ_pos.x - em_pos.x;
                // Angle from poition to target along the x-axis. So, anything +y will have a positive angle, -y will have neg angle.
                let mut targ_angle = y.atan2(x) * (180.0 / 3.14159265358979323846);
                // Set angle to correct value between 0 and 360
                if targ_angle < 0.0 { targ_angle = 360.0 + targ_angle;}
                // println!("target_angle: {}", targ_angle);
                let mut target_hit = false;

                // Is the target in the beam-width
                // If the emission width crosses the x-axis from either side
                if (em_pos.direction + (em.azimuth_width / 2.0)) >= 360.0 || (em_pos.direction - (em.azimuth_width / 2.0)) <= 0.0 {
                    if targ_angle <= em.azimuth_width / 2.0 {
                        targ_angle = targ_angle + 360.0;
                    }
                    if em_pos.direction >= 0.0 {
                        if(targ_angle - em_pos.direction - 360.0).abs() <= (em.azimuth_width / 2.0) {
                            target_hit = true;
                        }
                    }
                } 
                if (targ_angle - em_pos.direction).abs() <= (em.azimuth_width / 2.0)  {
                    target_hit = true;
                }

                if target_hit {
                    // Power received: Pr = (Pt * G^2 * lambda^2 * rcs) / ((4pi)^3 * R^4)
                    println!("!!!!Target Hit!!!!");
                    println!("Target Angle: {} at {}", targ_angle, targ_pos.x);
                    println!("Emission Direction: {}\tWidth: {}", em_pos.direction, em.azimuth_width);
                    let range = ((em_pos.x - targ_pos.x).powi(2) + (em_pos.y - targ_pos.y).powi(2)).sqrt();
                    let power = em.power / (4.0 * 3.14 * range.powi(2));
                    let new_abs = Illumination{power: power, lambda: em.wavelength, frequency: em.frequency, angle: targ_angle, rcs: targ_rcs.avg_rcs};
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