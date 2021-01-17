#[derive(Debug)]
pub struct Antenna {
    pub frequency: f32,         // Hz
    pub gain: f32,              // w / w
    pub power: f32,               // Watts
    pub wavelength: f32,            // wavelength
    pub elevation_beam_width: f32,    // degrees, We'll assume elevation is infinitley tall for now
    pub azimuth_beam_width: f32,      // degrees
}