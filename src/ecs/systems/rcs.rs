use super::*;

pub struct RCSSystem;
impl<'a> System<'a> for RCSSystem {
    type SystemData = (
        ReadStorage<'a, RCS>,
        WriteStorage<'a, TargetIllumination>,
    );

    fn run(&mut self, (cross_sections, mut illuminations) : Self::SystemData)  {
        for (rcs, targ) in (&cross_sections, &mut illuminations).join() {
            for ill in targ.illuminations.iter_mut() {
                ill.rcs = calculate_rcs(ill.angle, &rcs.angles, &rcs.values);
            }
        }
    }
}