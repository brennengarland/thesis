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