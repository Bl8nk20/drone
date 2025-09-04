/* 
Brainstorming:
GENERAL:
Coordinates as a 3 long Array: [x, y, z] -> float64 ? or float32
Logging with file

Design Patterns:
Singleton -> Espacially for Sensors and Filter

STRUCTS:
Struct for SensorDriver
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

Struct for Filtering & Adjusting:
 * PiD-Adjusting
 * Kallman-Filter
 */

mod components;
mod motor_driver;

fn main() {}
