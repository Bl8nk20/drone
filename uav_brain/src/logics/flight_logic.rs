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

fn match_pattern(pattern:FlightPatterns, engine_ref:&[Engine; 4]){
    match pattern{
        FlightPatterns::PitchFront => leaning(engine_ref, FlightPatterns::PitchFront),
        FlightPatterns::PitchBack => leaning(engine_ref, FlightPatterns::PitchBack),
        FlightPatterns::RollLeft=> leaning(engine_ref, FlightPatterns::RollLeft),
        FlightPatterns::RollRight => leaning(engine_ref, FlightPatterns::RollRight),
        FlightPatterns::YawLeft => rotate(engine_ref, FlightPatterns::YawLeft),
        FlightPatterns::YawRight => rotate(engine_ref, FlightPatterns::YawRight),
        FlightPatterns::Up => height_manipulations(engine_ref, FlightPatterns::Up),
        FlightPatterns::Down => height_manipulations(engine_ref, FlightPatterns::Down),
        _ => (),
    }    
}
/* 
Functions to rotate to either side. left, right
-> 
*/
fn rotate(engine_ref:&[Engine; 4], pattern: FlightPatterns){
    /*
    Adjust the Speeds of the Engines, which are turning in the same directions
    */
    match  pattern{
        FlightPatterns::YawLeft => ,
        FlightPatterns::YawRight => rotate(engine_ref),
        _ => (),
    };
}

/* 
Functions to lean to either side. front, back, left, right
-> MAX degree of leaning: 45°
*/
fn leaning(engine_ref:&[Engine; 4], pattern: FlightPatterns){
    /*
    Adjust the Engines to the opposite/same site of the desired leaning side 
    */
    match pattern{
        FlightPatterns::PitchFront => leaning(engine_ref),
        FlightPatterns::PitchBack => leaning(engine_ref),
        FlightPatterns::RollLeft=> leaning(engine_ref),
        FlightPatterns::RollRight => leaning(engine_ref),
        _ => (),
    }
}

/*
Function to fly up or down!
-> Float32 to adjust speeds to nearest int ? 
*/
fn height_manipulations(engine_ref:&[Engine; 4], pattern: FlightPatterns){
    /*
    Adjsut all speeds!
    */
    match pattern {
        FlightPatterns::Down => (),
        FlightPatterns::Up => (),
        _ => (),
    }
}
