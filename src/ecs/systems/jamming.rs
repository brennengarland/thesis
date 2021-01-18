use super::*;

pub struct JammingSystem;
impl<'a> System<'a> for JammingSystem {
    type SystemData = (
        ReadStorage<'a, TargetIllumination>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        ReadStorage <'a, Antenna>,
        Entities<'a>,
    );

    fn run(&mut self, (target_illumination, mut emission, mut position, antenna, entities) : Self::SystemData) {
        
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<EMWave> = Vec::new();
        // Iterate through each target
        for (target, pos, ant) in (&target_illumination, &mut position, &antenna).join() {
            for ill in target.illuminations.iter() {
                println!("New Jamming EM Wave: {}", ill.angle);
                let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
                let emission = EMWave{power: (ant.power*ant.gain), wavelength: ant.wavelength, frequency: ant.frequency, azimuth_width: ant.azimuth_beam_width, elevation_width: ant.elevation_beam_width};
                // println!("Emission Direction: {}", position.direction);
                new_positions.push(position);
                new_emissions.push(emission);
            }
        }

        while new_positions.len() != 0 {
            let new_entity = entities.create();
            match position.insert(new_entity, new_positions.remove(0)) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
            match emission.insert(new_entity, new_emissions.remove(0)) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
        }
    }
}