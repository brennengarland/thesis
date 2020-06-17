// extern crate specs;

use specs::prelude::*;
use std::{thread, time};

// A component contains data which is associated with an entity.
// pos in meters
#[derive(Debug)]
struct Position {
    x: f32,
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


struct RadarSensing;
impl<'a> System<'a> for RadarSensing {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Signature>,
        WriteStorage<'a, Targets>,
    );

    fn run(&mut self, (pos, sig, mut targ): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // You could also use `par_join()` to get a rayon `ParallelIterator`.
        for (targ, radar_pos) in (&mut targ, &pos).join() {
            for(targ_pos, sig) in (&pos, &sig).join() {
                
                let range = ((radar_pos.x - targ_pos.x).powi(2) + (radar_pos.y - targ_pos.y).powi(2)).sqrt();

                // // Begin range equation
                // let c = 299792458.0;
                // let d_t = 1.0;     // Delta t (s)
                // let range = (c*d_t) / 2.0;
                println!("\nTarget at {}m\n", range);
                targ.targ_array.push(range);
            }

        }
    }
}

struct Movement;
impl<'a> System<'a> for Movement {

    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut position, velocity): Self::SystemData) {
        for(pos, vel) in (&mut position, &velocity).join() {
            pos.x += vel.x;
            pos.y += vel.y;
            println!("X: {}", pos.x);
            println!("Y: {}", pos.y);
        }
    }
}

fn main() {
    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::new();

    // This builds a dispatcher
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new().with(RadarSensing, "radar_sensing", &[]).with(Movement, "movement", &[]).build();

    // setup() must be called before creating any entity, it will register
    // all Components and Resources that Systems depend on
    dispatcher.setup(&mut world);

    // An entity may or may not contain some component
    // This entity does not have `Vel`, so it won't be dispatched.
    let radar = world.create_entity().with(Position{x: 0.0, y: 0.0, z: 1.0}).with(Targets{targ_array: Vec::new()}).build();
    let target = world.create_entity().with(Position{x: 10.0, y: 20.0, z: 1.0}).with(Signature{0: 10.0}).with(Velocity{x: 10.0, y: 10.0, z: 0.0}).build();

    // This dispatches all the systems in parallel (but blocking)
    let one_sec = time::Duration::from_secs(1);
    loop {
        let start = time::Instant::now();
        dispatcher.dispatch(&world);
        world.maintain();
        let sleep_time = one_sec.checked_sub(time::Instant::now().duration_since(start));
        if sleep_time != None {
            thread::sleep(sleep_time.unwrap());
        }
        // thread::sleep(one_sec);
    }
}