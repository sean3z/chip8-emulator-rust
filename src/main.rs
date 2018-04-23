mod cpu;
mod keypad;
mod display;

mod system;
use system::System;

fn main() {
    let system = System::new();
    system.init();
    system.load("pong");

    loop {
        system.cycle();
        system.draw();

        break;
    }
}
