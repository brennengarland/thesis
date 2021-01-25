use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RCS {
    pub angles: Vec<f32>,
    pub values: Vec<f32>,
    pub avg_rcs: f32,
}