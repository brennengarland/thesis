use specs::prelude::*;
use crate::components::{RCS, TargetIllumination};

pub struct RCSSystem;
impl<'a> System<'a> for RCSSystem {
    type SystemData = (
        ReadStorage<'a, RCS>,
        WriteStorage<'a, TargetIllumination>,
    );

    fn run(&mut self, (cross_sections, mut illuminations) : Self::SystemData)  {
        for (rcs, targ) in (&cross_sections, &mut illuminations).join() {
            for ill in targ.illuminations.iter_mut() {
                let mut refl_pwr: f32 = -1.0;
                if rcs.angles.contains(&ill.angle) {
                    refl_pwr = rcs.values[rcs.angles.iter().position(|&r| r == ill.angle).unwrap()];
                } else {
                    for i in 0 .. rcs.angles.len()-1 {
                        if ill.angle < rcs.angles[i] {
                            refl_pwr = rcs.values[i];
                        }
                    }
    
                    if refl_pwr == -1.0 {
                        refl_pwr = rcs.values[rcs.values.len()-1];
                    }
                }
                ill.rcs = refl_pwr;

            }
        }
    }
}