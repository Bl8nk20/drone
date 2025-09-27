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
        FlightPatterns::PitchFront => pitch(engine_ref, true),
        FlightPatterns::PitchBack => pitch(engine_ref, false),
        FlightPatterns::RollLeft=> roll(engine_ref, FlightPatterns::RollLeft),
        FlightPatterns::RollRight => roll(engine_ref, FlightPatterns::RollRight),
        FlightPatterns::YawLeft => yaw(engine_ref, FlightPatterns::YawLeft),
        FlightPatterns::YawRight => yaw(engine_ref, FlightPatterns::YawRight),
        FlightPatterns::Up => height(engine_ref, true),
        FlightPatterns::Down => height(engine_ref, false),
        _ => (),
    }    
}

fn adjust_front(eng:&mut Engine, forward:bool, delta:i32){
    if let Some(curr) = eng.get_current() {
        eng.update_target(curr + if forward {-delta} else {delta});
    }
}

fn adjust_back(eng:&mut Engine, forward:bool, delta:i32){
    if let Some(curr) = eng.get_current() {
        eng.update_target(curr + if forward {delta} else {-delta});
    }}

fn match_pitch(eng:&mut Engine, forward:bool, delta:i32){
    match eng.orientation{
        Orientation::FrontLeft | Orientation::FrontRight => adjust_front(eng, forward, delta),
        Orientation::BackLeft | Orientation::BackRight => adjust_back(eng, forward, delta),
    }
}

fn pitch(engines:&mut [Engine;4], forward:bool){
    let delta = 100;
    for eng in engines.iter_mut(){
        match_pitch(eng, forward, delta);
    }
}

fn roll(engines:&mut [Engine;4], ){

}
fn yaw(engines:&mut [Engine;4], ){

}

fn height(engines:&mut [Engine;4], up:bool){
    let delta = if up{ 100} else { -100 };

    for eng in engines.iter_mut(){
        if let Some(curr) = eng.get_current(){
            eng.update_target((curr + delta).max(0));
        }
    }
}