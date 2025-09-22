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
use crate::motor_driver::engines::Engine;
use crate::components::Orientation;


// ONLY TEMPORARY !
const RPM_STEP: i32 = 100; // RPM change per movement for now

// Top-level flight command handler
pub fn match_pattern(pattern: FlightPatterns, engines: &mut [Engine; 4]) {
    match pattern {
        FlightPatterns::PitchFront => pitch_front(engines),
        FlightPatterns::PitchBack => pitch_back(engines),
        FlightPatterns::RollLeft => roll_left(engines),
        FlightPatterns::RollRight => roll_right(engines),
        FlightPatterns::YawLeft => yaw_left(engines),
        FlightPatterns::YawRight => yaw_right(engines),
        FlightPatterns::Up => increase_altitude(engines),
        FlightPatterns::Down => decrease_altitude(engines),
    }
}

fn get_orientation_index(engines: &[Engine; 4], target: Orientation) -> Option<usize> {
    engines.iter().position(|e| *e.get_orientation() == target)
}

// Yaw = Rotate (oppose diagonals to rotate)
fn yaw_left(engines: &mut [Engine; 4]) {
    // Increase RPM on FrontRight & BackLeft, Decrease on FrontLeft & BackRight
    adjust_rpm(engines, &[Orientation::FrontRight, Orientation::BackLeft], RPM_STEP);
    adjust_rpm(engines, &[Orientation::FrontLeft, Orientation::BackRight], -RPM_STEP);
}

fn yaw_right(engines: &mut [Engine; 4]) {
    // Opposite of yaw_left
    adjust_rpm(engines, &[Orientation::FrontLeft, Orientation::BackRight], RPM_STEP);
    adjust_rpm(engines, &[Orientation::FrontRight, Orientation::BackLeft], -RPM_STEP);
}

// Pitch = Lean forward or backward
fn pitch_front(engines: &mut [Engine; 4]) {
    // Increase back motors, decrease front
    adjust_rpm(engines, &[Orientation::BackLeft, Orientation::BackRight], RPM_STEP);
    adjust_rpm(engines, &[Orientation::FrontLeft, Orientation::FrontRight], -RPM_STEP);
}

fn pitch_back(engines: &mut [Engine; 4]) {
    adjust_rpm(engines, &[Orientation::FrontLeft, Orientation::FrontRight], RPM_STEP);
    adjust_rpm(engines, &[Orientation::BackLeft, Orientation::BackRight], -RPM_STEP);
}

// Roll = Lean left/right
fn roll_left(engines: &mut [Engine; 4]) {
    adjust_rpm(engines, &[Orientation::FrontRight, Orientation::BackRight], RPM_STEP);
    adjust_rpm(engines, &[Orientation::FrontLeft, Orientation::BackLeft], -RPM_STEP);
}

fn roll_right(engines: &mut [Engine; 4]) {
    adjust_rpm(engines, &[Orientation::FrontLeft, Orientation::BackLeft], RPM_STEP);
    adjust_rpm(engines, &[Orientation::FrontRight, Orientation::BackRight], -RPM_STEP);
}

// Up/Down
fn increase_altitude(engines: &mut [Engine; 4]) {
    adjust_rpm_all(engines, RPM_STEP);
}

fn decrease_altitude(engines: &mut [Engine; 4]) {
    adjust_rpm_all(engines, -RPM_STEP);
}

fn adjust_rpm_all(engines: &mut [Engine; 4], delta: i32) {
    for engine in engines.iter_mut() {
        let current = engine.get_current();
        engine.update_target(current + delta);
    }
}

fn adjust_rpm(engines: &mut [Engine; 4], targets: &[Orientation], delta: i32) {
    for engine in engines.iter_mut() {
        if targets.contains(&engine.get_orientation()) {
            let current = engine.get_current();
            engine.update_target(current + delta);
        }
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pitch_front() {
        use crate::components::Orientation;

        let mut engines = [
            Engine::new(["A"; 5].map(String::from), Orientation::FrontLeft),
            Engine::new(["B"; 5].map(String::from), Orientation::FrontRight),
            Engine::new(["C"; 5].map(String::from), Orientation::BackLeft),
            Engine::new(["D"; 5].map(String::from), Orientation::BackRight),
        ];

        match_pattern(FlightPatterns::PitchFront, &mut engines);

        assert_eq!(engines[2].get_values()[2], RPM_STEP); // BackLeft
        assert_eq!(engines[0].get_values()[2], -RPM_STEP); // FrontLeft
    }
}