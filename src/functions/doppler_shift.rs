use super::*;

pub fn doppler_shift(vel: &Velocity, illum: &Illumination) -> f32 {
    let tot_vel = (vel.x.powi(2) + vel.y.powi(2) + vel.z.powi(2)).sqrt();
    return (1.0 + (2.0 * (tot_vel / 300000000.0))) * illum.frequency; 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doppler_shift() {
        let vel = Velocity{
            x: 100.0, 
            y: 100.0, 
            z: 0.0
        };
        let illum = Illumination{
            angle: 100.0, 
            frequency: 10.0, 
            lambda: 100.0, 
            power: 50.0, 
            rcs: 1.0
        };
        assert_eq!(doppler_shift(&vel, &illum), 10.000009428090416);
    }
}