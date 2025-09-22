/*
Struct for Engine:
 * pins: Array[String]
 * values -> Dictionary with following Keys: [last, current, target] and Tuple(rpm, timestamp) 
 * orientation: Enum
 Methods:
 * get() -> Returning the Current_Val, if available
 * is_active() -> Returning, whether if the sensor is active or not
 * activate() -> Switch on the sensor
 * deactivate() -> switch off the sensor
 * update_target() -> updating only the target_value, e.g. when UAV reached the desired rpm
 * update_current() -> updating the current_value
 */

 // values[0] = current_rpm
 // values[1] = old_target
 // values[2] = new_target
 
use crate::components::Orientation;

pub struct Engine {
    pinout: [String; 5],
    orientation: Orientation,
    values: [i32; 3], // [current, old_target, new_target]
}

impl Engine {
    // Constructor
    pub fn new(pins: [String; 5], engine_orientation: Orientation) -> Self {
        Self {
            pinout: pins,
            orientation: engine_orientation,
            values: [0, 0, 0],
        }
    }

    // Get current RPM
    pub fn get_current(&self) -> i32 {
        self.values[0]
    }

    // Get pinout
    pub fn get_pins(&self) -> &[String; 5] {
        &self.pinout
    }
    
    // Optional: Get all values
    pub fn get_values(&self) -> &[i32; 3] {
        &self.values
    }

    pub fn get_orientation(&self) -> &Orientation {
        &self.orientation
    }

    // Update target RPM
    pub fn update_target(&mut self, new_target: i32) {
        self.values[1] = self.values[2]; // old_target = new_target
        self.values[2] = new_target;
    }

    // Update current RPM
    pub fn update_current(&mut self, current_rpm: i32) {
        self.values[0] = current_rpm;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Orientation;

    #[test]
    fn test_update_current() {
        let pinout_test: [String; 5] = ["A"; 5].map(String::from);
        let orientation_test = Orientation::FrontLeft;
        let mut engine = Engine::new(pinout_test, orientation_test);

        engine.update_current(220);
        assert_eq!(engine.get_current(), 220);
    }

    #[test]
    fn test_update_target() {
        let pinout_test: [String; 5] = ["B"; 5].map(String::from);
        let orientation_test = Orientation::FrontLeft;
        let mut engine = Engine::new(pinout_test, orientation_test);

        engine.update_target(3000);
        assert_eq!(engine.get_values()[1], 0);       // old_target (initial)
        assert_eq!(engine.get_values()[2], 3000);    // new_target

        engine.update_target(3500);
        assert_eq!(engine.get_values()[1], 3000);    // old_target = previous new_target
        assert_eq!(engine.get_values()[2], 3500);
    }
}
