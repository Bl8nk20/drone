/*
This is going to become the UAV part of the Program.
Here is going to be everthing related to the functionality of the UAV.

Things the UAV should send to the Ground Station:
- Up-To-Date information from various Sensors
Things the UAV should receive from the Ground Station:
- New Commands, what to do next
Things the UAV should be able to do:
- Adjust Engine rpm's when necessary
- Read Sensor Values at relative speeds
- Automatically land when a Specific time is reached ?!
*/

mod components;
mod sensor_builder;
mod motor_driver;
mod logics;

fn main() {
    println!("Hello, world!");
}
