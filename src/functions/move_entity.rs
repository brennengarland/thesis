use super::*;

pub fn move_entity(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x;
    pos.y += vel.y;
    pos.z += vel.z;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_move() {
//         let mut pos = Position{x: 10.0, y: 0.0, z: 1.0, direction: 0.0};
//         let vel = Velocity{x: 100.0, y: 100.0, z: 0.0};
//         move_entity(&mut pos, &vel);
//         assert_eq!(pos, Position{x: 110.0, y: 100.0, z: 1.0, direction: 0.0});
//     }
// }