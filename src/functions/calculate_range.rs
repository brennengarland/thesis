use super::*;


/// Calculates range using Euclidian distance
pub fn calculate_range(pos1: &Position, pos2: &Position) -> f32 {
    return ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2)).sqrt();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let radar = Position{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            direction: 0.0
        };

        let target = Position {
            x: 100.0,
            y: 100.0,
            z: 0.0,
            direction: 0.0
        };

        assert_eq!(calculate_range(&radar, &target), 141.42135623731);
    }
}


