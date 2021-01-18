use super::*;

pub struct AntennaReceiverSystem;
// Radar Sensor reads from environment
impl<'a> System<'a> for AntennaReceiverSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, EMWave>,
        ReadStorage<'a, Antenna>,
        Entities<'a>,
    );

    fn run(&mut self, (positions, emissions, antennas, entities) : Self::SystemData) {
        for (_antenna, antenna_pos) in (&antennas, &positions).join() {
            for(em_entity, em, em_pos) in (&*entities, &emissions, &positions).join() {
                let y = antenna_pos.y - em_pos.y;
                let x = antenna_pos.x - em_pos.x;
                let range = (y.powi(2) + x.powi(2)).sqrt();
                // Angle from poition to target along the x-axi&*s. So, anything +y will have a positive angle, -y will have neg angle.
                let mut targ_angle = y.atan2(x) * (180.0 / 3.14159265358979323846);

                // Set angle to correct value between 0 and 360
                if targ_angle < 0.0 { targ_angle = 360.0 + targ_angle;}
                // println!("target_angle: {}", targ_ang,le);
                let mut target_hit = false;
                
                if (em_pos.direction + (em.azimuth_width / 2.0)) >= 360.0 || (em_pos.direction - (em.azimuth_width / 2.0)) <= 0.0 {
                    if targ_angle <= em.azimuth_width / 2.0 {
                        targ_angle = targ_angle + 360.0;
                    }
                    if em_pos.direction >= 0.0 {
                        if(targ_angle - em_pos.direction - 360.0).abs() <= (em.azimuth_width / 2.0) {
                            target_hit = true;
                        }
                    }
                } 
                if (targ_angle - em_pos.direction).abs() <= (em.azimuth_width / 2.0)  {
                    target_hit = true;
                }

                if target_hit {
                    println!("Radar detected emission from angle: {}", antenna_pos.direction);
                    let _time = range / (3.0 * (100000000.0));
                    let _power = em.power;
                }
            
                match entities.delete(em_entity) {
                    Ok(r) => r,
                    Err(e) => eprintln!("Error!\n {}", e),
                }
            }
        }

    }
}