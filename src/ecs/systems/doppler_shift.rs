use super::*;

pub struct DopplerShiftSystem;
impl<'a> System<'a> for DopplerShiftSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, TargetIllumination>,
    );

    fn run(&mut self, (velocities, mut target_ills) : Self::SystemData) {
        for (vel, targ) in (&velocities, &mut target_ills).join() {
            for ill in targ.illuminations.iter_mut() {
                ill.frequency = doppler_shift(vel, ill);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doppler_shift() {
    }
}