use specs::prelude::*;


struct TransmitSignal;
impl<'a> System<'a> for TransmitSignal {
    type SystemData = (
        ReadStorage<'a, Antenna>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        Entities<'a>,
        Read<'a, LazyUpdate>
    );

    fn run(&mut self, (antennas, mut em_waves, mut positions, entities, updater): Self::SystemData) {
        // Must Read from each radar system and save values, then create the new emission afterwards
        // because we cannot iterate over positions and write to them at the same time.
        for (ant, pos) in (&antennas, &mut positions).join() {

            let new_pos = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
            let new_wave = EMWave{power: (ant.power*ant.gain), wavelength: ant.wavelength, frequency: ant.frequency, azimuth_width: ant.azimuth_beam_width, elevation_width: ant.elevation_beam_width};
            let new_entity = entities.create();
            updater.insert(new_entity, new_pos);
            updater.insert(new_entity, new_wave);
        }
    }
}