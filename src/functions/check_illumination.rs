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