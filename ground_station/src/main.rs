/*
This is the Ground Station Coding Part.
There are some things the Ground station should receive and send while the UAV is in the air.
This Workspace is going to be flexible and customizable, 
so that I could upgrade it for example, with a live feed of the uav

GOALS:
- Intuitive UI
- Commands via UI to UAV
- Encrypted, at least not easy to hack
- Flexible and Extendable
- Live Feed from UAV (video preparation, sensor must have)
- Possibility to map controls to another device (controller, smartphone, ...)

Things the Ground Station should send:
- Commands to the UAV with Custom Events
  - UP / Down commands
  - Land command
  - Left/Right commands
  - Rotate commands
  - Forward/ Backward commands

Things the Ground Station should receive:
- Up-to-Date information about sensor values [Barometer = Height, IMU = multiple things, ...]

Other Tasks for the Ground Station:
- Being a Interface for the UAV and the Human controlling it
-> Visible appealing, NO Terminal style view, maybe even a GUI ?! -> Iced as the GUI Lib !

Features to consider:
Peer-to-Peer connection for UAV and Ground Station
-> Encrypted messages! Data Safety!
*/


fn main() {
    println!("Hello, world!");
}
