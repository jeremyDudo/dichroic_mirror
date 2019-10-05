mod lib;
use lib::*;

fn main() {
    const NN1: f32 = 1.46;
    const NN2: f32 = 4.6;
    const DN1: f32 = 52.75;
    const DN2: f32 = 52.75;
    const DSUB: f32 = 1000.0;
    const TOTLAY: u32 = 101;

    quad_show(&NN1, &NN2, &DN1, &DN2, &DSUB, &TOTLAY);

}