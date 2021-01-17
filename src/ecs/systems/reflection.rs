use super::*;

// Creates an emission from the absorption information
pub struct ReflectionSystem;
impl<'a> System<'a> for ReflectionSystem {
    type SystemData = (
        WriteStorage<'a, TargetIllumination>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        Entities<'a>,
    );

    fn run(&mut self, (mut target_illumination, mut emission, mut position, entities) : Self::SystemData) {
        
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<EMWave> = Vec::new();
        // Iterate through each target
        for (target, pos) in (&mut target_illumination, &position).join() {
            for ill in target.illuminations.iter() {
                let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: (180.0 + ill.angle) % 360.0};
                let p_r = ill.power * ill.rcs;
                let emission = EMWave{power: p_r, wavelength: ill.lambda, frequency: ill.frequency, azimuth_width: 20.0, elevation_width: 20.0};
                // println!("Emission Direction: {}", position.direction);
                new_positions.push(position);
                new_emissions.push(emission);
            }
            target.illuminations.clear();
        }

        while new_positions.len() != 0 {
            let new_entity = entities.create();
            // println!("Emission Direction: {}", position.direction);
            position.insert(new_entity, new_positions.remove(0));
            emission.insert(new_entity, new_emissions.remove(0));
        }
    }
}