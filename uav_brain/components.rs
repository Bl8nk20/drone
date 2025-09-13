/*
ENUMS:
Enum for SensorTypes = 
[IMU, Gyro, Baro, Magnetometer, etc]
Enum for Orientation of Engines = 
[FrontLeft, FrontRight, BackLeft, BackRight] -> maybe engines with enum calling?
Enum for flight-patterns = 
[YawLeft // RotateLeft, YawRight // RotateRight, PitchFront // Forward, PitchBack // Backwards, RollLeft //Left, RollRight //Right, Up, Down]

*/

#[derive(Debug)]
pub enum SensorType{
    Gyroscope,
    Accelerometer,
    IMU,
    Barometer,
    Magnetometer
} 

#[derive(Debug)]
pub enum Orientation{
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight
}
