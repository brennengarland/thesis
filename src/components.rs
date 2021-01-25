
use specs::{Component};
use crate::structs::*;

mod position;
mod em_wave;
mod antenna;
mod target_illumination;
mod rcs;
mod velocity;

impl Component for Antenna {
    type Storage = VecStorage<Self>;
}

impl Component for EMWave {
    type Storage = VecStorage<Self>;
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for RCS {
    type Storage = VecStorage<Self>;
}

impl Component for TargetIllumination {
    type Storage = VecStorage<Self>;
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
