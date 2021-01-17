#[derive(Debug)]
pub struct EMWave {
    pub power: f32,
    pub wavelength: f32,
    pub frequency: f32,
    pub azimuth_width: f32,     // Degrees
    pub elevation_width: f32
}