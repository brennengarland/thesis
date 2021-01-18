use super::*;
use std::f32::consts::PI;

/// Returns the angle, in degrees, between two points, an emitter and target, from the perspective of the emitter
pub fn incident_angle(emitter: &Position, target: &Position) -> f32 {
    let y_diff = target.y - emitter.y;
    let x_diff = target.x - emitter.x;
    let angle = y_diff.atan2(x_diff) * (180.0 / PI);
    if angle < 0.0 {
        return 360.0 + angle
    } else {
        return angle;
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
}