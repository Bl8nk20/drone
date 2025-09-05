/* 
Brainstorming:
GENERAL:
Coordinates as a 3 long Array: [x, y, z] -> float64 ? or float32
Logging with file

Design Patterns:
Singleton -> Especially for Sensors and Filter
Factory -> For Sensor, Motor registry
Command -> Factory Calling, to sort of hot swapping sensors, or enginges, etc

STRUCTS:
Struct for SensorDriver

Struct for Filtering & Adjusting:
 * PiD-Adjusting
 * Kallman-Filter
 */

mod components;
mod motor_driver;
mod sensor;

fn main() {}
