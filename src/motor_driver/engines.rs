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

use std::collections::HashMap;
use crate::components::Orientation;

pub struct Engine{
    pinout:[String; 5],
    orientation: Orientation,
    values: HashMap<String, i32>,
}
impl Engine{
    // build a new Engine- Object
    pub const fn new(pins: [String; 5], engine_orientation: Orientation, value_dict:HashMap<String, i32> ) -> Self{
        return Self{ pinout:pins, orientation: engine_orientation, values:value_dict};
    }
    // returning the current rpm-parameters
    pub fn get_current(&self) -> &i32{
        return self.values.get("current_rpm").unwrap_or(&0);
    }

    pub fn get_pins(&self) -> &[String; 5] {
        return  &self.pinout;
    }

    // Something like that to update the new value at the Key!
    pub fn update_target(mut self, new_target:i32){
        self.values.insert(String::from("old_rpm_target"), self.values.get("target_rpm").copied().unwrap_or(0));
        self.values.insert(String::from("target_rpm"), new_target);
    }

    fn update_current(mut self, current_rpm : i32){
        self.values.insert(String::from("target_rpm"), current_rpm);
    }
}