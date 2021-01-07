
use specs::prelude::*;
use specs::{Component};
use serde::{Deserialize};


#[derive(Debug)]
pub struct Position {
    x: f32, // meters
    y: f32,
    z: f32,
    direction: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct EMWave {
    power: f32,
    wavelength: f32,
    frequency: f32,
    azimuth_width: f32,     // Degrees
    elevation_width: f32
}
impl Component for EMWave {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Antenna {
    frequency: f32,         // Hz
    gain: f32,              // w / w
    power: f32,               // Watts
    wavelength: f32,            // wavelength
    elevation_beam_width: f32,    // degrees, We'll assume elevation is infinitley tall for now
    azimuth_beam_width: f32,      // degrees
}
impl Component for Antenna {
    type Storage = VecStorage<Self>;
}

pub struct TargetIllumniation {
    illuminations: Vec<Illumniation>,
}

impl Component for TargetIllumniation {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Illumniation {
    power: f32,
    lambda: f32,
    frequency: f32,
    angle: f32,
    rcs: f32,
}

#[derive(Debug, Deserialize)]
pub struct RCS {
    angles: Vec<f32>,
    values: Vec<f32>,
    avg_rcs: f32,
}

impl Component for RCS {
    type Storage = VecStorage<Self>;
}

// m/s
#[derive(Debug)]
pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}