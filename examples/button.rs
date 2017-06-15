extern crate libbeaglebone;

use libbeaglebone::gpio::*;

fn main() {
  // Create a GPIO object at pin #66 that'll represent the button, export it,
  // and set it as an input
  // Adjust the pin number to whatever pin your LED is connected to
  let button = GPIO::new(66);
  button.set_export(true).unwrap();
  button.set_direction(PinDirection::In).unwrap();
  println!("Waiting for button press...");

  for _ in 1..6 {
    // Wait for button to be hit and then released 5 times
    while button.read().unwrap() != PinState::High {}
    println!("Button hit!");
    while button.read().unwrap() != PinState::Low {}
    println!("Button released!");
  }
}
