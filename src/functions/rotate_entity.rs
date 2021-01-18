use super::*;

pub fn rotate_entity(pos: &mut Position, sen: &Antenna) {
    pos.direction = (pos.direction + sen.azimuth_beam_width / 2.0) % 360.0;

}