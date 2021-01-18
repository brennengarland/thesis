// use super::*;

/// Returns true if an angle is in a range, returns false otherwise
pub fn check_illumination(em_width: f32, em_dir: f32, input_angle: f32) -> bool {
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



#[cfg(test)]
mod tests {
    use super::*;

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