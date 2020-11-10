// extern crate specs;

use specs::prelude::*;
use specs::{Component, Entities};
use std::{thread, time, fs};
use specs::Join;
use serde_json::{Value, Map};


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
struct EMWave {
    power: f32,
    wavelength: f32,
    frequency: f32,
    azimuth_width: f32,     // Degrees
    elevation_width: f32
}
impl Component for EMWave {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Antenna {
    frequency: f32,         // Hz
    gain: f32,              // w / w
    power: f32,               // Watts
    wavelength: f32,            // wavelength
    elevation_beam_width: f32,    // degrees, We'll assume elevation is infinitley tall for now
    azimuth_beam_width: f32,      // degrees
}
impl Component for Antenna {
    type Storage = VecStorage<Self>;
}


struct TransmitSignal;
impl<'a> System<'a> for TransmitSignal {
    type SystemData = (
        ReadStorage<'a, Antenna>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        Entities<'a>,
        Read<'a, LazyUpdate>
    );

    fn run(&mut self, (sensors, mut em_waves, mut positions, entities, updater): Self::SystemData) {
        // Must Read from each radar system and save values, then create the new emission afterwards
        // because we cannot iterate over positions and write to them at the same time.
        for (sen, pos) in (&sensors, &mut positions).join() {
            // println!("\nRadar Direction: {}", pos.direction);

            let new_pos = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
            let new_wave = EMWave{power: (sen.power*sen.gain), wavelength: sen.wavelength, frequency: sen.frequency, azimuth_width: sen.azimuth_beam_width, elevation_width: sen.elevation_beam_width};
            
            let new_entity = entities.create();
            updater.insert(new_entity, new_pos);
            updater.insert(new_entity, new_wave);
        }
    }
}

struct TargetIllumniation {
    illuminations: Vec<Illumniation>,
}

impl Component for TargetIllumniation {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Illumniation {
    power_density: f32,
    lambda: f32,
    frequency: f32,
    angle: f32,
}

// Detects Interactions
struct InteractionDetection;
impl<'a> System<'a> for InteractionDetection {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, EMWave>,
        WriteStorage<'a, TargetIllumniation>,
        Entities<'a>,
    );


    fn run(&mut self, (positions, emissions, mut illumination, entities): Self::SystemData) {
        // let threshold_power = 0.0;

        // Loop through all of the emissions. em_entity is just an identifier
        for (em_entity, em, em_pos) in (&*entities, &emissions, &positions).join() {
            // println!("Emission Direction: {}", emission_origin.direction);

            // Loops through entities with only a position and illumination. Should just be our 'targets'
            for(targ_pos, ill) in (&positions, &mut illumination).join() {

                let y = targ_pos.y - em_pos.y;
                let x = targ_pos.x - em_pos.x;
                // Angle from poition to target along the x-axis. So, anything +y will have a positive angle, -y will have neg angle.
                let mut targ_angle = y.atan2(x) * (180.0 / 3.14159265358979323846);
                // Set angle to correct value between 0 and 360
                if targ_angle < 0.0 { targ_angle = 360.0 + targ_angle;}
                // println!("target_angle: {}", targ_angle);
                let mut target_hit = false;

                // Is the target in the beam-width
                // If the emission width crosses the x-axis from either side
                if (em_pos.direction + em.azimuth_width) >= 360.0 || (em_pos.direction - em.azimuth_width) <= 0.0 {
                    if (360.0 % (targ_angle - em_pos.direction).abs()) <= (em.azimuth_width / 2.0) {
                        target_hit = true;
                    }
                // Else the emission does not cross he x-axis, just check if it's in the arc
                } else if (targ_angle - em.azimuth_width).abs() <= (em.azimuth_width / 2.0)  {
                    target_hit = true;
                }

                if target_hit {
                    println!("Target Location: {}", targ_angle);
                    // Power received: Pr = (Pt * G^2 * lambda^2 * rcs) / ((4pi)^3 * R^4)
                    let range = ((em_pos.x - targ_pos.x).powi(2) + (em_pos.y - targ_pos.y).powi(2)).sqrt();
    
                    // let p_r = (emission.p_t * emission.gain.powi(2) * sig.0 * emission.lambda.powi(2) * 10.0_f32.powi(14)) / (1984.4017 * range.powi(4));
                    // let tot_v = (targ_vel.x.powi(2) + targ_vel.y.powi(2)).sqrt();
                    // let f_r = (1.0 + 2.0 * (tot_v / 300000000.0)) * emission.frequency;
                    // println!("Received Power: {}\nReceived Frequency: {}", p_r, f_r);
                    let power_density = em.power / (4.0 * 3.14 * range.powi(2));
                    let new_abs = Illumniation{power_density: power_density, lambda: em.wavelength, frequency: em.frequency, angle: targ_angle};
                    ill.illuminations.push(new_abs);
                }
            }
            println!("Deleting Target at ({},{})", em_pos.x, em_pos.y);
            match entities.delete(em_entity) {
                Ok(r) => r,
                Err(e) => eprintln!("Error!\n {}", e),
            }
        }
    }
}

struct DopplerShiftSystem;
impl<'a> System<'a> for DopplerShiftSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, TargetIllumniation>,
    );

    fn run(&mut self, (velocities, mut target_ills) : Self::SystemData) {
        for (vel, targ) in (&velocities, &mut target_ills).join() {
            for ill in targ.illuminations.iter_mut() {
                let tot_vel = (vel.x.powi(2) + vel.y.powi(2) + vel.z.powi(2)).sqrt();
                let f_r = (1.0 + (2.0 * (tot_vel / 300000000.0))) * ill.frequency;
                ill.frequency = f_r;
            }
        }
    }
}

#[derive(Debug)]
struct RCS(f32);
impl Component for RCS {
    type Storage = VecStorage<Self>;
}

// Creates an emission from the absorption information
struct ReflectionSystem;
impl<'a> System<'a> for ReflectionSystem {
    type SystemData = (
        WriteStorage<'a, TargetIllumniation>,
        WriteStorage<'a, EMWave>,
        WriteStorage<'a, Position>,
        ReadStorage <'a, RCS>,
        Entities<'a>,
    );

    fn run(&mut self, (mut target_illumination, mut emission, mut position, rcs, entities) : Self::SystemData) {
        
        let mut new_positions: Vec<Position> = Vec::new();
        let mut new_emissions: Vec<EMWave> = Vec::new();
        // Iterate through each target
        for (target, pos, target_rcs) in (&mut target_illumination, &position, &rcs).join() {
            for ill in target.illuminations.iter() {
                println!("New Target Illumanted at angle: {}", ill.angle);
                let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
                let p_r = ill.power_density * target_rcs.0;
                let emission = EMWave{power: p_r, wavelength: ill.lambda, frequency: ill.frequency, azimuth_width: 20.0, elevation_width: 20.0};
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
        ReadStorage<'a, EMWave>,
        ReadStorage<'a, Antenna>,
        Entities<'a>,
    );

    fn run(&mut self, (positions, emissions, antennas, entities) : Self::SystemData) {
        for (antenna, antenna_pos) in (&antennas, &positions).join() {
            for(em_entity, em, em_pos) in (&*entities, &emissions, &positions).join() {
                println!("Emission Pos: ({}, {})", em_pos.x, em_pos.y);

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
                if (em_pos.direction + em.azimuth_width) >= 360.0 || (em_pos.direction - em.azimuth_width) <= 0.0 {
                    if (360.0 % (targ_angle - em_pos.direction).abs()) <= (em.azimuth_width / 2.0) {
                        target_hit = true;
                    }
                // Else the emission does not cross the x-axis, just check if it's in the arc
                } else if (targ_angle - em.azimuth_width).abs() <= (em.azimuth_width / 2.0)  {
                    target_hit = true;
                }

                if target_hit {
                    println!("Radar detected emission from angle: {}", targ_angle);
                }
            
                match entities.delete(em_entity) {
                    Ok(r) => r,
                    Err(e) => eprintln!("Error!\n {}", e),
                }
            }
        }

    }
}

// m/s
#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
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
            position.direction = (position.direction + sensor.azimuth_beam_width) % max_direction;

            // println!("Radar Direction: {}", position.direction);

        }
    }
}

fn main() {

    let mut world = World::new();
    let mut transmission = DispatcherBuilder::new()
    .with(TransmitSignal, "transmit_signal", &[]).build();
    transmission.setup(&mut world);

    let mut illumination = DispatcherBuilder::new()
    .with(InteractionDetection, "radar_sensing", &[]).build();
    illumination.setup(&mut world);

    let mut reflection = DispatcherBuilder::new()
    .with(DopplerShiftSystem, "doppler_shift", &[])
    .with(ReflectionSystem, "reflection_creation", &["doppler_shift"]).build();
    reflection.setup(&mut world);

    let mut reception = DispatcherBuilder::new()
    .with(AntennaReceiverSystem, "antenna_receiver", &[])
    .with(Movement, "movement", &[]).build();
    reception.setup(&mut world);

    // INPUTS FOR RADAR SENSOR
    let p_t: f32 = 100.0;           // kW    
    let gain = 32.0;                // dB
    let frequency = 9400000000.0;   // Hz

    // TARGET INFO
    let rcs = 1.0;                 // m^2
    let targ_x = 50000.0;          // m from sensor
    let targ_y = -100.0;
    let targ_z = 100.0;

    // An entity may or may not contain some component
    let _radar = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0, direction: 0.0})
    .with(Antenna{
        frequency: frequency, 
        gain: 10.0_f32.powf(gain / 10.0), 
        power: (p_t * 1000.0), 
        wavelength: ((3.0 * 100000000.0) / frequency),
        azimuth_beam_width: 20.0,
        elevation_beam_width: 20.0,
        }).build();

    let _target1 = world.create_entity()
    .with(Position{x: targ_x, y: targ_y, z: targ_z, direction: 0.0})
    .with(RCS{0: rcs})
    .with(Velocity{x: 0.0, y: 0.0, z: 0.0})
    .with(TargetIllumniation{illuminations: Vec::new(),})
    .build();


    let runtime = time::Duration::from_secs(1);
    // About 60 frames / sec
    // let runtime = time::Duration::from_micros(16)
    loop {
        let start = time::Instant::now();
        // TransmitSignal.run_now(&world);
        transmission.dispatch(&world);
        world.maintain();
        illumination.dispatch(&world);
        world.maintain();
        reflection.dispatch(&world);
        world.maintain();
        reception.dispatch(&world);
        world.maintain();
        // Create frame_rate loop
        let sleep_time = runtime.checked_sub(time::Instant::now().duration_since(start));

        let contents = fs::read_to_string("reflection_geometry.txt")
        .expect("Something went wrong reading the file");

        println!("{}", contents);
        
        if sleep_time != None {
            thread::sleep(sleep_time.unwrap());
        }
    }
}