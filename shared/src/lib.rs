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
