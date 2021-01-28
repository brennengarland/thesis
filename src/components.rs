
use specs::{Component};
use specs::prelude::*;
use crate::structs::*;

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
