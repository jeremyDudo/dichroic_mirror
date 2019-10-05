mod lib;
use lib::*;

fn main() {
    // TODO: Make more manipulable, we aren't taking advantage of this SPEED
    const NN1: f32 = 1.46;
    const NN2: f32 = 4.6;
    const DN1: f32 = 52.75;
    const DN2: f32 = 52.75;
    const DSUB: f32 = 1000.0;
    const TOTLAY: u32 = 21;

    quad_show(&NN1, &NN2, &DN1, &DN2, &DSUB, &TOTLAY);

}