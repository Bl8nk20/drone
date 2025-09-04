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

 pub struct Engine{
    
 }