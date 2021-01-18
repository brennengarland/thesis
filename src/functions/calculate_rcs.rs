// use super::*;

pub fn calculate_rcs(angle: f32, rcs_angles: &Vec<f32>, rcs_values: &Vec<f32>) -> f32 {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rcs() {
        let angle = [0.0, 10.0, 100.0, 355.0];
        let angles = vec![0.0, 10.0, 200.0];
        let values = vec![10.0, 50.0, 100.0];
        let truth = [10.0, 50.0, 100.0, 100.0];

        for n in 0..angle.len() {
            assert_eq!(calculate_rcs(angle[n], &angles, &values), truth[n]);
        }
    }
}