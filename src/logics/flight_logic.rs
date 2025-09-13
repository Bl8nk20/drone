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

use crate::components::FlightPatterns;
use crate::motor_driver::engines::Engine;

fn match_pattern(pattern:FlightPatterns, engine_ref:&[Engine; 4]){
    match pattern{
        FlightPatterns::PitchFront => lean_front(engine_ref),
        FlightPatterns::PitchBack => lean_back(engine_ref),
        FlightPatterns::RollLeft=> lean_left(engine_ref),
        FlightPatterns::RollRight => lean_right(engine_ref),
        FlightPatterns::YawLeft => rotate_left(engine_ref),
        FlightPatterns::YawRight => rotate_right(engine_ref),
        FlightPatterns::Up => up(engine_ref),
        FlightPatterns::Down => down(engine_ref),
        _ => (),
    }    
}
/* 
Functions to rotate to either side. left, right
-> 
*/
fn rotate(engine_ref:&[Engine; 4]){
    /*
    Adjust the Speeds of the Engines, which are turning in the same directions
    */
}
fn rotate_left(engine_ref:&[Engine; 4]){
    rotate(engine_ref);
}
fn rotate_right(engine_ref:&[Engine; 4]){
    rotate(engine_ref);
}

/* 
Functions to lean to either side. front, back, left, right
-> MAX degree of leaning: 45°
*/
fn leaning(engine_ref:&[Engine; 4]){
    /*
    Adjust the Engines to the opposite/same site of the desired leaning side 
    */
}

fn lean_front(engine_ref:&[Engine; 4]){
    leaning(engine_ref)
}
fn lean_back(engine_ref:&[Engine; 4]){
    leaning(engine_ref);
}

fn lean_right(engine_ref:&[Engine; 4]){
    leaning(engine_ref);
}
fn lean_left(engine_ref:&[Engine; 4]){
    leaning(engine_ref);
}
/*
Function to fly up or down!
-> Float32 to adjust speeds to nearest int ? 
*/
fn height_manipulations(engine_ref:&[Engine; 4]){
 /*
 Adjsut all speeds!
 */
}

fn up(engine_ref:&[Engine; 4]){
    height_manipulations(engine_ref);
}

fn down(engine_ref:&[Engine; 4]){
    height_manipulations(engine_ref);
}