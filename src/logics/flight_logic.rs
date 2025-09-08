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
        _ => (),
    }
}

/* 
Functions to rotate to either side. left, right
*/

fn rotate(){

}

fn rotateLeft(){

}

fn rotateRight(){

}

/* 
Functions to lean to either side. front, back, left, right
*/

fn leaning(){

}

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
*/

fn zMovement(){

}

fn upwards(){

}

fn downwards(){

}