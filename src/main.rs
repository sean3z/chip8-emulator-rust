extern crate rand;
extern crate sdl;
extern crate winconsole;

// use std::io;
use sdl::event::Event;

use cpu::Cpu;

mod cpu;
mod keypad;
mod display;


fn main() {
    let mut cpu = Cpu::new();

    cpu.load_game("/home/sean/www/chip8-emulator-rust/roms/pong");

    sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

    'main : loop {
        'event : loop {
            match sdl::event::poll_event() {
                Event::Quit                  => break 'main,
                Event::None                  => break 'event,
                Event::Key(key, state, _, _) => cpu.keypad.press(key, state),
                _                            => {}
            }
        }

        cpu.emulate_cycle();
        cpu.display.draw_screen();
    }

    sdl::quit();
}
