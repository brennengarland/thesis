// extern crate specs;

use specs::prelude::*;

// A component contains data which is associated with an entity.
#[derive(Debug)]
struct Pos {
    x: f32,
    y: f32,
    z: f32,
}
impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Signature(f32);
impl Component for Signature {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Range(f32);
impl Component for Range {
    type Storage = VecStorage<Self>;
}

struct Targets {
    targ_array: Vec<f32>,
}
impl Component for Targets {
    type Storage = VecStorage<Self>;
}


struct RadarSensing;
impl<'a> System<'a> for RadarSensing {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Signature>,
        WriteStorage<'a, Targets>,
    );

        fn run(&mut self, (pos, sig, mut targ): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.
        // You could also use `par_join()` to get a rayon `ParallelIterator`.
        for (targ) in (&mut targ).join() {
            for(pos, sig) in (&pos, &sig).join() {
                println!("Position: {}", pos.x);
                println!("Signature: {}", sig.0);

                // Begin range equation
                let c = 299792458.0;
                let d_t = 1.0;     // Delta t (s)
                let range = (c*d_t) / 2.0;
                targ.targ_array.push(range);
            }

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
    let mut dispatcher = DispatcherBuilder::new().with(RadarSensing, "radar_sensing", &[]).build();

    // setup() must be called before creating any entity, it will register
    // all Components and Resources that Systems depend on
    dispatcher.setup(&mut world);

    // An entity may or may not contain some component
    // This entity does not have `Vel`, so it won't be dispatched.
    let radar = world.create_entity().with(Pos{x: 0.0, y: 0.0, z: 1.0}).with(Targets{targ_array: Vec::new()}).build();
    let target = world.create_entity().with(Pos{x: 10.0, y: 20.0, z: 1.0}).with(Signature{0: 10.0}).build();

    // This dispatches all the systems in parallel (but blocking).
    dispatcher.dispatch(&world);
}