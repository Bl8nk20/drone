/*
This File should contain every necessary Method and Struct to ensure flying behavior.
The Logic is getting called by Enums in the Components.rs file.

Functions:
* YawLeft,     // RotateLeft
* YawRight,    // RotateRight
* PitchFront,  // Forward
* PitchBack,   // Backwards
* RollLeft,    // Left
* RollRight,   // Right
* Up,          // Upwards
* Down         // Downwards

* case Matching to call Behaviour ! 
*/

use shared::FlightPatterns;

use crate::shared::FlightPatterns;
use crate::motor_driver::engines::Engine;

pub fn match_pattern(pattern:FlightPatterns, engine_ref:&[Engine; 4]){
    match pattern{
        FlightPatterns::PitchFront => pitch(engine_ref, FlightPatterns::PitchFront),
        FlightPatterns::PitchBack => pitch(engine_ref, FlightPatterns::PitchBack),
        FlightPatterns::RollLeft=> roll(engine_ref, FlightPatterns::RollLeft),
        FlightPatterns::RollRight => roll(engine_ref, FlightPatterns::RollRight),
        FlightPatterns::YawLeft => yaw(engine_ref, FlightPatterns::YawLeft),
        FlightPatterns::YawRight => yaw(engine_ref, FlightPatterns::YawRight),
        FlightPatterns::Up => height(engine_ref, FlightPatterns::Up),
        FlightPatterns::Down => height(engine_ref, FlightPatterns::Down),
        _ => (),
    }    
}

fn pitch(engines:&mut [Engine;4], pattern:FlightPatterns){

}

fn roll(engines:&mut [Engine;4], pattern:FlightPatterns){

}
fn yaw(engines:&mut [Engine;4], pattern:FlightPatterns){

}

fn height(engines:&mut [Engine;4], pattern:FlightPatterns){

}