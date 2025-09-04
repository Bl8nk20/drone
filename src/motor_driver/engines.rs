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
 * update_targets() -> updating last, current & target values
 * update_target() -> updating only the target_value, e.g. when UAV reached the desired rpm
 */

use structures::map::Dictionary;
use crate::components::Orientation;

pub struct Engine{
    pinout:[String; 5],
    orientation: Orientation,
    values: Option<Dictionary<i32>>,
}
impl Engine{
    pub const fn new(pins: [String; 5], engine_orientation: Orientation, value_dict: Option<Dictionary<i32>>) -> Self{
        return Self{ pinout:pins, orientation: engine_orientation, values:value_dict};
    }
}