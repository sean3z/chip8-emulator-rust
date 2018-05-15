mod cpu;
mod keypad;
mod display;

mod system;
use system::System;

fn main() {
    let mut system = System::new("/Users/sean/Downloads/c8games/PONG");

    loop {
        system.cycle();

        break;
    }
}
