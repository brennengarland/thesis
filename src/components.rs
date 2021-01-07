
use specs::prelude::*;
use specs::{Component};
use serde::{Deserialize};


#[derive(Debug)]
pub struct Position {
    pub x: f32, // meters
    pub y: f32,
    pub z: f32,
    pub direction: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct EMWave {
    pub power: f32,
    pub wavelength: f32,
    pub frequency: f32,
    pub azimuth_width: f32,     // Degrees
    pub elevation_width: f32
}
impl Component for EMWave {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Antenna {
    pub frequency: f32,         // Hz
    pub gain: f32,              // w / w
    pub power: f32,               // Watts
    pub wavelength: f32,            // wavelength
    pub elevation_beam_width: f32,    // degrees, We'll assume elevation is infinitley tall for now
    pub azimuth_beam_width: f32,      // degrees
}
impl Component for Antenna {
    type Storage = VecStorage<Self>;
}

pub struct TargetIllumination {
    pub illuminations: Vec<Illumination>,
}

impl Component for TargetIllumination {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Illumination {
    pub power: f32,
    pub lambda: f32,
    pub frequency: f32,
    pub angle: f32,
    pub rcs: f32,
}

#[derive(Debug, Deserialize)]
pub struct RCS {
    pub angles: Vec<f32>,
    pub values: Vec<f32>,
    pub avg_rcs: f32,
}

impl Component for RCS {
    type Storage = VecStorage<Self>;
}

// m/s
#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}