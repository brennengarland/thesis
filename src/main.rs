// extern crate specs;

use specs::prelude::*;
use std::{thread, time};


#[derive(Debug)]
struct Position {
    x: f32, // meters
    y: f32,
    z: f32,
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
    frequency: f32, // Hz
    gain: f32,      // w / w
    p_t: f32,       // Watts
    lambda: f32,    // wavelength
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

// Simulating the radar sensor
struct RadarSensing;
impl<'a> System<'a> for RadarSensing {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Signature>,
        ReadStorage<'a, Sensor>,
    );

    fn run(&mut self, (pos, sig, sen): Self::SystemData) {
        // Loops through entities with only a target list and posiiton
        for (sen, radar_pos) in (&sen, &pos).join() {
            // Loops through entities with only a position and signiture
            for(pos, sig) in (&pos, &sig).join() {
            
            // Power received: Pr = (Pt * G^2 * lambda^2 * rcs) / ((4pi)^3 * R^4)
            let r = ((radar_pos.x - pos.x).powi(2) + (radar_pos.y - pos.y).powi(2)).sqrt();

            let p_r = (sen.p_t * sen.gain.powi(2) * sig.0 * sen.lambda.powi(2) * 10.0_f32.powi(14)) / (1984.4017 * r.powi(4));

            println!("Received Power: {}", p_r);
            }

        }
    }
}

// Changes the position of each entity with position and velocity
struct Movement;
impl<'a> System<'a> for Movement {

    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut position, velocity): Self::SystemData) {
        let mut i = 1;
        for(pos, vel) in (&mut position, &velocity).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            println!("Target{}\nX: {}", i, pos.x);
            println!("Y: {}", pos.y);
            i += 1;
        }
    }
}

fn main() {

    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new().with(RadarSensing, "radar_sensing", &[]).build();
    dispatcher.setup(&mut world);

    // INPUTS FOR RADAR SENSOR
    
    let p_t: f32 = 100.0;        // kW    
    let gain = 32.0;                // dB
    let frequency = 9400000000.0;  // Hz

    // TARGET INFO
    let rcs = 1.0;                 // m^2
    let targ_x = 50000.0;          // m from sensor
    let targ_y = 0.0;
    let targ_z = 0.0;

    // An entity may or may not contain some component
    let _radar = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0})
    .with(Sensor{
        frequency: frequency, 
        gain: 10.0_f32.powf(gain / 10.0), 
        p_t: (p_t * 1000.0), 
        lambda: ((3.0 * 100000000.0) / frequency),
        }).build();
    let _target1 = world.create_entity().with(Position{x: targ_x, y: targ_y, z: targ_z}).with(Signature{0: rcs}).build();

    // This dispatches all the systems in parallel (but blocking)
    let one_sec = time::Duration::from_secs(1);
    loop {
        let start = time::Instant::now();
        dispatcher.dispatch(&world);
        // Create frame_rate loop
        let sleep_time = one_sec.checked_sub(time::Instant::now().duration_since(start));
        if sleep_time != None {
            thread::sleep(sleep_time.unwrap());
        }
    }
}