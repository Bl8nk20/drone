use std::time::SystemTime;

/*
Structs for Sensors (IMU, Gyro, GPS, Baro?)
 Varibales:
 * pins: Array[String]
 * type: Enum
 * lastVal: T
 * currentVal: T
 * timestamp: Systemtime
 * is_active: bool
 * update_rate_hz: f32
 * callibrated: bool
 Methods:
 * new() -> Setting variables, creating the Object
 * update() -> Reading from the Sensor
 * convert() -> Converting to DT for better handling
 * get() -> Returning the Current_Val, if available
 * is_active() -> Returning, whether if the sensor is active or not
 * activate() -> Switch on the sensor
 * deactivate() -> switch off the sensor
 * callibrate() -> callibrating the sensor
*/
use crate::components::SensorType;


pub struct Sensor{
    pinout:[String; 5],
    typ: SensorType,
    last_val: f32,
    current_val:f32,
    timestamp: SystemTime,
    is_active: bool,
    update_rate_hz: f32,
    callibrated:bool
}
impl Sensor{

}