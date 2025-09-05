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

use structures::map::Dictionary;
use crate::components::Orientation;

pub struct Engine{
    pinout:[String; 5],
    orientation: Orientation,
    values: Option<Dictionary<i32>>,
}
impl Engine{
    // build a new Engine- Object
    pub const fn new(pins: [String; 5], engine_orientation: Orientation, value_dict: Option<Dictionary<i32>>) -> Self{
        return Self{ pinout:pins, orientation: engine_orientation, values:value_dict};
    }
    // returning the current rpm-parameters
    pub const fn getValues(&self) -> &Option<Dictionary<i32>>{
        return &self.values;
    }

    // Something like that to update the new value at the Key!
    pub const fn update_target(&self, new_target:i32){
        &self.values["old_rpm_target"] = &self.values["target_rpm"];
        &self.values["target_rpm"] = new_target;
    }

    pub const fn update_current(&self, current_rpm : i32){
        &self.values["current_rpm"] = current_rpm;
    }
}