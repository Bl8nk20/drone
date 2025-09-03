/* 
Brainstorming:
Coordinates as a 3 long Array: [x, y, z]
Enum for SensorTypes = [IMU, Gyro, Baro, Magnetometer, etc]
Struct for SensorDriver
Structs for Sensors (IMU, Gyro, GPS, Baro?)
 Varibales:
 * pins: Array[String]
 * type: Enum
 * Value: Enum
 * unit: Option<String>
 * lastVal: T
 * currentVal: T
 * timestamp: Systemtime
 * is_active: bool
 * update_rate_hz: f32
 Methods:
 * new() -> Setting variables, creating the Object
 * update() -> Reading from the Sensor
 * convert() -> Converting to DT for better handling
 * get() -> Returning the Current_Val, if available
 * is_active() -> Returning, whether if the sensor is active or not
 * activate() -> Switch on the sensor
 * deactivate() -> switch off the sensor
Struct for Engine:
 * 
Struct for Filtering & Adjusting:
 * PiD-Adjusting
 * Kallman-Filter
 */

fn main() {}
