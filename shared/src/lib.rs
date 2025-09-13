/*
This is a shared lib for the Ground Station and the UAV
Here and maybe the following structs, files, modules or crates
are coming the parts, where both parts need to know
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum FlightPatterns {
    YawLeft,
    YawRight,
    PitchFront,
    PitchBack,
    RollLeft,
    RollRight,
    Up,
    Down,
}
