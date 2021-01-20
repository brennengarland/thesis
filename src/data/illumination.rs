#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Illumination {
    pub power: f32,
    pub lambda: f32,
    pub frequency: f32,
    pub angle: f32,
    pub rcs: f32,
}