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

pub struct Engine{
    pinout:[String; 5],
    orientation: Orientation,
    values: Option<[i32; 3]>,
}
impl Engine{
    // build a new Engine- Object
    pub const fn new(pins: [String; 5], engine_orientation: Orientation) -> Self{
        return Self{ pinout:pins, orientation: engine_orientation, values:None};
    }
    // returning the current rpm-parameters
    pub fn get_current(&self) -> &i32{
        //return self.values.get().expect("current")
        return &3;
    }

    pub fn get_pins(&self) -> &[String; 5] {
        return  &self.pinout;
    }

    // Something like that to update the new value at the Key!
    pub fn update_target(self, new_target:i32){
        self.values.unwrap_or([0,0,0])[0] = self.values.unwrap()[2];
        self.values.unwrap_or([0,0,0])[2] = new_target;
    }

    pub fn update_current(mut self, current_rpm : i32){
        self.values.unwrap_or([0,0,0])[2] = current_rpm;
    }
}