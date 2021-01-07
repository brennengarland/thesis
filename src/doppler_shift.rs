


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