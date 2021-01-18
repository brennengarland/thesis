use specs::*;


#[test]
fn test_transmission() {
    let mut world = World::new();
    let mut transmission = DispatcherBuilder::new()
    .with(TransmitSignal, "transmit_signal", &[]).build();
    transmission.setup(&mut world);

    // INPUTS FOR RADAR SENSOR
    let p_t: f32 = 100.0;           // kW    
    let gain = 32.0;                // dB
    let frequency = 9400000000.0;   // Hz

    // TARGET INFO
    let _rcs = 1.0;                 // m^2
    let targ_x = 100.0;          // m from sensor
    let targ_y = 50.0;
    let targ_z = 0.0;

    // RCS 
    let data = fs::read_to_string("src/data.json").expect("Unable to read file");
    // Parse the string of data into serde_json::Value.
    let targ_rcs: RCS = serde_json::from_str(&data).expect("error parsing");
    // println!("Avg RCS: {}", targ_rcs.avg_rcs);

    // An entity may or may not contain some component
    let _radar: specs::Entity = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0, direction: 5.0})
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
    .with(TargetIllumination{illuminations: Vec::new(),})
    // .with(Velocity{x: -50.0, y: -100.0, z: -10.0})
    .build();

    TransmitSignal.run_now(&world);
    world.
}