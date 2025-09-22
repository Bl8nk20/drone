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
use iced::{widget::{button, column, text, Column}, Error};

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}

fn main() -> Result<(), Error> {
    // Some logic here
    let _ = iced::run("Counter", Counter::update, Counter::view);
    
    // Ensure you return an appropriate Result
    Ok(()) // or Err(some_error)
}

