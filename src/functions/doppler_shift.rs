use super::*;

pub fn doppler_shift(vel: &Velocity, illum: &Illumination) -> f32 {
    let tot_vel = (vel.x.powi(2) + vel.y.powi(2) + vel.z.powi(2)).sqrt();
    return (1.0 + (2.0 * (tot_vel / 300000000.0))) * illum.frequency; 
}