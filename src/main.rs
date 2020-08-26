// extern crate specs;

use specs::prelude::*;
use specs::Entities;
use std::{thread, time, num};


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
struct Signature(f32);
impl Component for Signature {
    type Storage = VecStorage<Self>;
}

struct Targets {
    targ_array: Vec<f32>,
}
impl Component for Targets {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Sensor {
    frequency: f32,         // Hz
    gain: f32,              // w / w
    p_t: f32,               // Watts
    lambda: f32,            // wavelength
    pulse_width: f32,       // microseconds
    // elevation_beam: f32,    // degrees, We'll assume elevation is infinitley tall for now
    azimuth_beam: f32,      // degrees
    dwell_time: f32,        // milliseconds
}
impl Component for Sensor {
    type Storage = VecStorage<Self>;
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

#[derive(Debug)]
struct Emission {
    p_t: f32,
    lambda: f32,
    frequency: f32,
    gain: f32,
}
impl Component for Emission {
    type Storage = VecStorage<Self>;
}

struct TransmitSignal;
impl<'a> System<'a> for TransmitSignal {
    type SystemData = (
        ReadStorage<'a, Sensor>,
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
            let position = Position{x: pos.x, y: pos.y, z: pos.z, direction: pos.direction};
            let emission = Emission{p_t: sen.p_t, lambda: sen.lambda, frequency: sen.frequency, gain: sen.gain};

            new_positions.push(position);
            new_emissions.push(emission);
        }

        while new_positions.len() != 0 {
            let new_entity = entities.create();
            println!("Number of positions: {}", new_positions.len());
            positions.insert(new_entity, new_positions.remove(0));
            emissions.insert(new_entity, new_emissions.remove(0));
        }
        println!("Number of positions: {}", new_positions.len());

    }
}

// Simulating the radar sensor
struct RadarSensing;
impl<'a> System<'a> for RadarSensing {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Signature>,
        ReadStorage<'a, Sensor>,
    );


    fn run(&mut self, (pos, sig, sen): Self::SystemData) {
        let thresholdPower = 0.0;

        // Loops through entities with only a sensor and posiiton
        for (sen, radar_pos) in (&sen, &pos).join() {

            let max_angle = (radar_pos.direction + (sen.azimuth_beam / 2.0)) % 360.0 ;
            let min_angle = radar_pos.direction - (sen.azimuth_beam / 2.0) % 360.0 ;

            // Loops through entities with only a position and signiturepul
            for(pos, sig) in (&pos, &sig).join() {

                let y = pos.y - radar_pos.y;
                let x = pos.x - radar_pos.x;
                // Angle from poition to target along the x-axis. So, anything +y will have a positive angle, -y will have neg angle.
                let mut targ_angle = y.atan2(x) * (180.0 / 3.14159265358979323846);

                if targ_angle < 0.0 { targ_angle = 360.0 + targ_angle;}

                // Is the target in the beam-width
                if (radar_pos.direction + sen.azimuth_beam) >= 360.0 || (radar_pos.direction - sen.azimuth_beam) <= 0.0 {
                    if (360.0 % (targ_angle - radar_pos.direction).abs()) <= (sen.azimuth_beam / 2.0) {
                        println!("Radar Found!\nTarget Angle: {}\nRadar Direction: {}", targ_angle, radar_pos.direction);
                    }
                } else if (targ_angle - radar_pos.direction).abs() <= (sen.azimuth_beam / 2.0)  {
                    println!("Radar Found!\nTarget Angle: {}\nRadar Direction: {}", targ_angle, radar_pos.direction);
                }

                // Power received: Pr = (Pt * G^2 * lambda^2 * rcs) / ((4pi)^3 * R^4)
                let r = ((radar_pos.x - pos.x).powi(2) + (radar_pos.y - pos.y).powi(2)).sqrt();

                let p_r = (sen.p_t * sen.gain.powi(2) * sig.0 * sen.lambda.powi(2) * 10.0_f32.powi(14)) / (1984.4017 * r.powi(4));

                // println!("Received Power: {}", p_r);
            }

        }
    }
}

// Changes the position of each entity with position and velocity
struct Movement;
impl<'a> System<'a> for Movement {

    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Sensor>,
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
            position.direction = (position.direction + sensor.azimuth_beam) % 360.0;

            // println!("Radar Direction: {}", position.direction);

        }
    }
}

fn main() {

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(RadarSensing, "radar_sensing", &[]).with(TransmitSignal, "transmit_signal", &[]).with(Movement, "movement", &[]).build();
    dispatcher.setup(&mut world);

    // INPUTS FOR RADAR SENSOR
    let p_t: f32 = 100.0;           // kW    
    let gain = 32.0;                // dB
    let frequency = 9400000000.0;   // Hz

    // TARGET INFO
    let rcs = 1.0;                 // m^2
    let targ_x = 50000.0;          // m from sensor
    let targ_y = -100.0;
    let targ_z = 0.0;

    // An entity may or may not contain some component
    let _radar = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0, direction: 0.0})
    .with(Sensor{
        frequency: frequency, 
        gain: 10.0_f32.powf(gain / 10.0), 
        p_t: (p_t * 1000.0), 
        lambda: ((3.0 * 100000000.0) / frequency),
        pulse_width: 10.0,
        azimuth_beam: 20.0,
        dwell_time: 10.0,
        }).build();

    let _target1 = world.create_entity().with(Position{x: targ_x, y: targ_y, z: targ_z, direction: 0.0}).with(Signature{0: rcs}).build();


    let runtime = time::Duration::from_secs(1);
    // About 60 frames / sec
    // let runtime = time::Duration::from_micros(16)
    loop {
        let start = time::Instant::now();
        dispatcher.dispatch(&world);
        // Create frame_rate loop
        let sleep_time = runtime.checked_sub(time::Instant::now().duration_since(start));
        if sleep_time != None {
            thread::sleep(sleep_time.unwrap());
        }
    }
}