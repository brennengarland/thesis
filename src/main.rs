// extern crate specs;

use specs::prelude::*;
use specs::{Component, Entities};
use std::{thread, time, fs};
use specs::Join;
use serde_json::{Value, Map};
use serde::{Deserialize};


mod antenna_receiver;
mod transmit_signal;

struct TargetIllumniation {
    illuminations: Vec<Illumniation>,
}

impl Component for TargetIllumniation {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Illumniation {
    power: f32,
    lambda: f32,
    frequency: f32,
    angle: f32,
    rcs: f32,
}

#[derive(Debug, Deserialize)]
struct RCS {
    angles: Vec<f32>,
    values: Vec<f32>,
    avg_rcs: f32,
}

impl Component for RCS {
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

fn main() {

    let mut world = World::new();
    let mut transmission = DispatcherBuilder::new()
    .with(transmit_signal::TransmitSignal, "transmit_signal", &[]).build();
    transmission.setup(&mut world);

    let mut illumination = DispatcherBuilder::new()
    .with(InteractionDetection, "radar_sensing", &[]).build();
    illumination.setup(&mut world);

    let mut reflection = DispatcherBuilder::new()
    .with(DopplerShiftSystem, "doppler_shift", &[])
    .with(RCSSystem, "rcs_system", &[])
    .with(ReflectionSystem, "reflection_creation", &["doppler_shift", "rcs_system"]).build();
    reflection.setup(&mut world);

    let mut reception = DispatcherBuilder::new()
    .with(antenna_receiver::AntennaReceiverSystem, "antenna_receiver", &[])
    .with(Movement, "movement", &[]).build();
    reception.setup(&mut world);

    // INPUTS FOR RADAR SENSOR
    let p_t: f32 = 100.0;           // kW    
    let gain = 32.0;                // dB
    let frequency = 9400000000.0;   // Hz

    // TARGET INFO
    let rcs = 1.0;                 // m^2
    let targ_x = 100.0;          // m from sensor
    let targ_y = 50.0;
    let targ_z = 0.0;

    // RCS 
    let data = fs::read_to_string("src/data.json").expect("Unable to read file");
    // Parse the string of data into serde_json::Value.
    let targ_rcs: RCS = serde_json::from_str(&data).expect("error parsing");
    // println!("Avg RCS: {}", targ_rcs.avg_rcs);

    // An entity may or may not contain some component
    let _radar = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0, direction: 5.0})
    .with(Antenna{
        frequency: frequency, 
        gain: 10.0_f32.powf(gain / 10.0), 
        power: (p_t * 1000.0), 
        wavelength: ((3.0 * 100000000.0) / frequency),
        azimuth_beam_width: 10.0,
        elevation_beam_width: 20.0,
        }).build();

    let _target1 = world.create_entity()
    .with(Position{x: targ_x, y: targ_y, z: targ_z, direction: 0.0})
    .with(targ_rcs)
    .with(Velocity{x: -10.0, y: 0.0, z: 0.0})
    .with(TargetIllumniation{illuminations: Vec::new(),})
    // .with(Velocity{x: -50.0, y: -100.0, z: -10.0})
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
        // let sleep_time = runtime.checked_sub(time::Instant::now().duration_since(start));
        
        // if sleep_time != None {
        //     thread::sleep(sleep_time.unwrap());
        // }
        thread::sleep(time::Duration::from_millis(500));
    }
}