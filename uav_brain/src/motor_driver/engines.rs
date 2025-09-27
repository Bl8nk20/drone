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

#[derive(Debug, Clone, Copy)]
pub struct EngineValues{
    pub last: i32,
    pub current: i32,
    pub target: i32,
}

pub struct Engine{
    pinout:[String; 5],
    orientation: Orientation,
    values: Option<EngineValues>,
}
impl Engine{
    // build a new Engine- Object
    pub const fn new(pins: [String; 5], engine_orientation: Orientation) -> Self{
        return Self{ pinout:pins, orientation: engine_orientation, values:None};
    }
    // returning the current rpm-parameters
    pub fn get_current(&self) -> &i32{
        return self.values.map(|v| v.current);
    }

    pub fn get_pins(&self) -> &[String; 5] {
        return  &self.pinout;
    }

    pub fn is_active(&self) {
        self.values.is_some()
    }

    pub fn activate(&mut self, start_rpm:i32){
        self.values = Some(EngineValues { last: 0,
            current: start_rpm,
            target: start_rpm });
    }

    pub fn deactivate(&mut self) {
        self.values = None;
    }

    // Something like that to update the new value at the Key!
    pub fn update_target(&mut self, new_target:i32){
        if let Some(v) = &mut self.values{
            v.last = v.target;
            v.target = new_target;
        }    
    }

    pub fn update_current(mut self, current_rpm : i32){
        if let Some(v) = &mut self.values{
            v.current = current_rpm;
        }
    }
}