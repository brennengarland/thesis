use specs::prelude::*;
use crate::components::{Position, EMWave, TargetIllumination, RCS, Illumination};
use std::f32::consts::PI;

/// Returns the angle, in degrees, between two points, an emitter and target, from the perspective of the emitter
fn incident_angle(emitter: &Position, target: &Position) -> f32 {
    let y_diff = target.y - emitter.y;
    let x_diff = target.x - emitter.x;
    let angle = y_diff.atan2(x_diff) * (180.0 / PI);
    if angle < 0.0 {
        return 360.0 + angle
    } else {
        return angle;
    }
}

/// Returns true if an angle is in a range, returns false otherwise
fn check_illumination(em_width: f32, em_dir: f32, input_angle: f32) -> bool {
    let mut angle = input_angle;
    if (em_dir + (em_width / 2.0)) >= 360.0 || (em_dir - (em_width / 2.0)) <= 0.0 {
        if angle <= em_width / 2.0 {
            angle = angle + 360.0;
        }
        if em_dir >= 0.0 {
            if(angle - em_dir - 360.0).abs() <= (em_width / 2.0) {
                return true;
            }
        }
    } 
    if (angle - em_dir).abs() <= (em_width / 2.0)  {
        return true;
    }

    return false;
}

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
                let mut angle = incident_angle(em_pos, targ_pos);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        let emitter = Position{x: 0.0, y: 0.0, z: 0.0, direction: 0.0};
        let target = Position{x: -50.0, y: -100.0, z: 0.0, direction: 180.0};
        assert_eq!(incident_angle(&emitter, &target), 243.43497);
    }

    #[test]
    fn test_illumination() {
        let width = 10.0;
        let direction = [45.0, 270.0, 180.0, 180.0, 3.0];
        let angle = [45.0, 270.0, 10.0, 185.0, 359.9];
        let truth_values = [true, true, false, true, true];
        for n in 0..direction.len() {
            assert_eq!(check_illumination(width, direction[n], angle[n]), truth_values[n]);   
        }
    }
}