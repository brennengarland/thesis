use super::*;

fn calculate_rcs(angle: f32, rcs_angles: &Vec<f32>, rcs_values: &Vec<f32>) -> f32 {
    let mut refl_pwr: f32 = -1.0;
    // Check to see if the rcs has a power specified for the angle
    if rcs_angles.contains(&angle) {
        refl_pwr = rcs_values[rcs_angles.iter().position(|&r| r == angle).unwrap()];
    } else {
        // Loop through all the angles to match the closest angle
        for i in 0 .. rcs_angles.len()-1 {
            if angle < rcs_angles[i] {
                refl_pwr = rcs_values[i];
            }
        }
        // If for some reason angle was higher than the highest mapped angle
        if refl_pwr == -1.0 {
            // Just set the refl power = to the power of the highest angle
            refl_pwr = rcs_values[rcs_values.len()-1];
        }
    }
    return refl_pwr;
}

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