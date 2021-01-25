#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
pub struct Antenna {
    pub frequency: f32,         // Hz
    pub gain: f32,              // w / w
    pub power: f32,               // Watts
    pub wavelength: f32,            // wavelength
    pub elevation_beam_width: f32,    // degrees, We'll assume elevation is infinitley tall for now
    pub azimuth_beam_width: f32,      // degrees
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct EMWave {
    pub power: f32,
    pub wavelength: f32,
    pub frequency: f32,
    pub azimuth_width: f32,     // Degrees
    pub elevation_width: f32
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Illumination {
    pub power: f32,
    pub lambda: f32,
    pub frequency: f32,
    pub angle: f32,
    pub rcs: f32,
}

#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
pub struct Position {
    pub x: f32, // meters
    pub y: f32,
    pub z: f32,
    pub direction: f32,
}

use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RCS {
    pub angles: Vec<f32>,
    pub values: Vec<f32>,
    pub avg_rcs: f32,
}

#[derive(Debug)]
pub struct TargetIllumination {
    pub illuminations: Vec<Illumination>,
}

// m/s
#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}