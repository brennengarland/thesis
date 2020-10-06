// extern crate specs;

use specs::prelude::*;
use specs::{Component, Entities};
use std::{thread, time};
use specs::Join;


#[derive(Debug)]
struct Position {
    x: f32, // meters
    y: f32,
    z: f32,
    direction: f32,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct RCS(f32);
impl Component for RCS {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Antenna {
    frequency: f32,         // Hz
    gain: f32,              // w / w
    p_t: f32,               // Watts
    lambda: f32,            // wavelength
    pulse_width: f32,       // microseconds
    // elevation_beam: f32,    // degrees, We'll assume elevation is infinitley tall for now
    azimuth_beam: f32,      // degrees
    dwell_time: f32,        // milliseconds
}
impl Component for Antenna {
    type Storage = VecStorage<Self>;
}

// m/s
// #[derive(Debug)]
// struct Velocity {
//     x: f32,
//     y: f32,
//     z: f32,
// }
// impl Component for Velocity {
//     type Storage = VecStorage<Self>;
// }

#[derive(Debug)]
struct Emission {
    power: f32,
    lambda: f32,
    frequency: f32,
    width: f32,     // Degrees
}
impl Component for Emission {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Illumniation {
    power_density: f32,
    lambda: f32,
    frequency: f32,
    angle: f32,
}

struct TargetIllumniation {
    illuminations: Vec<Illumniation>,
}

impl Component for TargetIllumniation {
    type Storage = VecStorage<Self>;
}


struct TransmitSignal;
impl<'a> System<'a> for TransmitSignal {
    type SystemData = (
        ReadStorage<'a, Antenna>,
        WriteStorage<'a, Emission>,
        WriteStorage<'a, Position>,
        Entities<'a>,
    );

    fn run(&mut self, (sensors, mut emissions, mut positions, entities): Self::SystemData) {
        // Must Read from each radar system and save values, then create the new emission afterwards
        // because we cannot iterate over positions and write to them at the same time.
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<Emission> = Vec::new();
        for (sen, pos) in (&sensors, &mut positions).join() {
            // println!("\nRadar Direction: {}", pos.direction);

            let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
            let emission = Emission{power: (sen.p_t*sen.gain), lambda: sen.lambda, frequency: sen.frequency, width: sen.azimuth_beam};
            // println!("Emission Direction: {}", position.direction);
            new_positions.push(position);
            new_emissions.push(emission);
        }

        while new_positions.len() != 0 {
            let new_entity = entities.create();
            // println!("Emission Direction: {}", position.direction);
            positions.insert(new_entity, new_positions.remove(0));
            emissions.insert(new_entity, new_emissions.remove(0));
        }

    }
}

// Detects Interactions
struct InteractionDetection;
impl<'a> System<'a> for InteractionDetection {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Emission>,
        WriteStorage<'a, TargetIllumniation>,
        Entities<'a>,
    );


    fn run(&mut self, (positions, emissions, mut illumination, entities): Self::SystemData) {
        // let threshold_power = 0.0;

        // Loops through entities with only a sensor and posiiton
        // println!("\n");
        for (em_entity, em, em_pos) in (&*entities, &emissions, &positions).join() {
            // println!("Emission Direction: {}", emission_origin.direction);
            // Loops through entities with only a position and signiturepul
            for(targ_pos, ill) in (&positions, &mut illumination).join() {

                let y = targ_pos.y - em_pos.y;
                let x = targ_pos.x - em_pos.x;
                // Angle from poition to target along the x-axi&*s. So, anything +y will have a positive angle, -y will have neg angle.
                let mut targ_angle = y.atan2(x) * (180.0 / 3.14159265358979323846);
                // Set angle to correct value between 0 and 360
                if targ_angle < 0.0 { targ_angle = 360.0 + targ_angle;}
                // println!("target_angle: {}", targ_angle);
                let mut target_hit = false;

                // Is the target in the beam-width
                // If the emission width crosses the x-axis from either side
                if (em_pos.direction + em.width) >= 360.0 || (em_pos.direction - em.width) <= 0.0 {
                    if (360.0 % (targ_angle - em_pos.direction).abs()) <= (em.width / 2.0) {
                        target_hit = true;
                    }
                // Else the emission does not cross he x-axis, just check if it's in the arc
                } else if (targ_angle - em.width).abs() <= (em.width / 2.0)  {
                    target_hit = true;
                }

                if target_hit {
                    println!("\nFound Target!\nRadar Direction: {}\nRadar Width: {}\nTarget Location: {}", em_pos.direction, em.width, targ_angle);
                    // Power received: Pr = (Pt * G^2 * lambda^2 * rcs) / ((4pi)^3 * R^4)
                    let range = ((em_pos.x - targ_pos.x).powi(2) + (em_pos.y - targ_pos.y).powi(2)).sqrt();
    
                    // let p_r = (emission.p_t * emission.gain.powi(2) * sig.0 * emission.lambda.powi(2) * 10.0_f32.powi(14)) / (1984.4017 * range.powi(4));
                    // let tot_v = (targ_vel.x.powi(2) + targ_vel.y.powi(2)).sqrt();
                    // let f_r = (1.0 + 2.0 * (tot_v / 300000000.0)) * emission.frequency;
                    // println!("Received Power: {}\nReceived Frequency: {}", p_r, f_r);
                    let power_density = em.power / (4.0 * 3.14 * range.powi(2));
                    let new_abs = Illumniation{power_density: power_density, lambda: em.lambda, frequency: em.frequency, angle: targ_angle};
                    ill.illuminations.push(new_abs);
                }
            }
            
            match entities.delete(em_entity) {
                Ok(r) => r,
                Err(e) => eprintln!("Error!\n {}", e),
            }
        }
    }
}

// Creates an emission from the absorption information
struct ReflectionSystem;
impl<'a> System<'a> for ReflectionSystem {
    type SystemData = (
        WriteStorage<'a, TargetIllumniation>,
        WriteStorage<'a, Emission>,
        WriteStorage<'a, Position>,
        ReadStorage <'a, RCS>,
        Entities<'a>,
    );

    fn run(&mut self, (mut target_illumination, mut emission, mut position, rcs, entities) : Self::SystemData) {
        
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<Emission> = Vec::new();
        // Iterate through each target
        for (target, pos, target_rcs) in (&mut target_illumination, &position, &rcs).join() {
            for ill in target.illuminations.iter() {
                println!("New Absorption Angle: {}", ill.angle);
                let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
                let p_r = ill.power_density * target_rcs.0;
                let emission = Emission{power: p_r, lambda: ill.lambda, frequency: ill.frequency, width: 0.0};
                // println!("Emission Direction: {}", position.direction);
                new_positions.push(position);
                new_emissions.push(emission);
            }
            target.illuminations.clear();
        }

        while new_positions.len() != 0 {
            let new_entity = entities.create();
            // println!("Emission Direction: {}", position.direction);
            position.insert(new_entity, new_positions.remove(0));
            emission.insert(new_entity, new_emissions.remove(0));
        }
    }
}


// Radar Sensor reads from environment
struct AntennaReceiverSystem;
impl<'a> System<'a> for AntennaReceiverSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Emission>,
        ReadStorage<'a, Antenna>,
        Entities<'a>,
    );

    fn run(&mut self, (positions, emissions, antennas, entities) : Self::SystemData) {
        for (antenna, antenna_pos) in (&antennas, &positions).join() {
            for(em, em_pos) in (&emissions, &positions).join() {

                let y = antenna_pos.y - em_pos.y;
                let x = antenna_pos.x - em_pos.x;
                // Angle from poition to target along the x-axi&*s. So, anything +y will have a positive angle, -y will have neg angle.
                let mut targ_angle = y.atan2(x) * (180.0 / 3.14159265358979323846);
                // Set angle to correct value between 0 and 360
                if targ_angle < 0.0 { targ_angle = 360.0 + targ_angle;}
                // println!("target_angle: {}", targ_angle);
                let mut target_hit = false;

                // Is the target in the beam-width
                // If the emission width crosses the x-axis from either side
                if (em_pos.direction + em.width) >= 360.0 || (em_pos.direction - em.width) <= 0.0 {
                    if (360.0 % (targ_angle - em_pos.direction).abs()) <= (em.width / 2.0) {
                        target_hit = true;
                    }
                // Else the emission does not cross the x-axis, just check if it's in the arc
                } else if (targ_angle - em.width).abs() <= (em.width / 2.0)  {
                    target_hit = true;
                }

                if(target_hit) {
                    println!("Radar detected emission");
                }
            }
        }

    }
}

// Changes the position of each entity with position and velocity
struct Movement;
impl<'a> System<'a> for Movement {

    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Antenna>,
    );

    fn run(&mut self, (mut position, mut sensor): Self::SystemData) {
        for(position, sensor) in (&mut position, &mut sensor).join() {
            let max_direction = 360.0;  // degrees

            // Rotate the beam between 0 and 360
            // position.direction += sensor.azimuth_beam;
            // if position.direction > max_direction  || position.direction <= 0.0 {
            //     sensor.azimuth_beam = sensor.azimuth_beam*-1.0;
            //     position.direction += sensor.azimuth_beam*2.0;
            // } 
            position.direction = (position.direction + sensor.azimuth_beam) % max_direction;

            // println!("Radar Direction: {}", position.direction);

        }
    }
}

fn main() {

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
    .with(TransmitSignal, "transmit_signal", &[])
    .with(InteractionDetection, "radar_sensing", &["transmit_signal"])
    .with(ReflectionSystem, "reflection_creation", &["radar_sensing"])
    .with(AntennaReceiverSystem, "antenna_receiver", &["reflection_creation"])
    .with(Movement, "movement", &[]).build();
    dispatcher.setup(&mut world);

    // INPUTS FOR RADAR SENSOR
    let p_t: f32 = 100.0;           // kW    
    let gain = 32.0;                // dB
    let frequency = 9400000000.0;   // Hz

    // TARGET INFO
    // let rcs = 1.0;                 // m^2
    let targ_x = 50000.0;          // m from sensor
    let targ_y = -100.0;
    let targ_z = 0.0;

    // An entity may or may not contain some component
    let _radar = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0, direction: 0.0})
    .with(Antenna{
        frequency: frequency, 
        gain: 10.0_f32.powf(gain / 10.0), 
        p_t: (p_t * 1000.0), 
        lambda: ((3.0 * 100000000.0) / frequency),
        pulse_width: 10.0,
        azimuth_beam: 20.0,
        dwell_time: 10.0,
        }).build();

    let _target1 = world.create_entity()
    .with(Position{x: targ_x, y: targ_y, z: targ_z, direction: 0.0})
    // .with(Signature{0: rcs})
    // .with(Velocity{x: 0.0, y: 0.0, z: 0.0})
    .with(TargetIllumniation{illuminations: Vec::new(),})
    .build();


    let runtime = time::Duration::from_secs(1);
    // About 60 frames / sec
    // let runtime = time::Duration::from_micros(16)
    loop {
        let start = time::Instant::now();
        dispatcher.dispatch(&world);
        world.maintain();
        // Create frame_rate loop
        let sleep_time = runtime.checked_sub(time::Instant::now().duration_since(start));
        if sleep_time != None {
            thread::sleep(sleep_time.unwrap());
        }
    }
}