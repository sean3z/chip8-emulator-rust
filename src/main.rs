mod cpu;
mod keypad;
mod display;

mod system;
use system::System;

fn main() {
    let mut system = System::new();
    system.init("pong");

    loop {
        system.cycle();
        system.draw();

        break;
    }
}
