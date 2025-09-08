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

fn matchPattern(pattern:FlightPatterns){
    match pattern{
        FlightPatterns::YawLeft => rotateLeft(),
        FlightPatterns::YawRight => rotateRight(),
        FlightPatterns::PitchFront => leanForward(),
        FlightPatterns::PitchBack => leanBackward(),
        FlightPatterns::RollLeft => leanLeft(),
        FlightPatterns::RollRight => leanRight(),
        FlightPatterns::Up => upwards(),
        FlightPatterns::Down => downwards(),
        _ => (),
    }
}

/* 
Functions to rotate to either side. left, right
-> 
*/

fn rotateLeft(){

}

fn rotateRight(){

}

/* 
Functions to lean to either side. front, back, left, right
-> MAX degree of leaning: 45°
*/

fn leanForward(){

}

fn leanBackward(){

}

fn leanLeft(){

}

fn leanRight(){

}

/*
Functions to fly up or down!
-> Float32 to adjust speeds to nearest int ? 
*/
fn height_manipulations(increaseby:f32){

}

fn upwards(value:f32){
 // Increase all Engine speeds by the same amount, if necessary adjust all to hover!
 height_manipulations(value);
}

fn downwards(){
 // Reduce all Engine speeds by the same amount, if necessary adjust all to hover!
}