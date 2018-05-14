use display::{Display, FONTS};
use cpu::Cpu;
use std::io::prelude::*;
use std::fs::File;
use keypad::Keypad;

pub struct System {
    cpu: Cpu,
    memory: [u8; 4096],
    keypad: Keypad,
    display: Display
}

impl System {
    pub fn new() -> System {
        System {
            cpu: Cpu::new(),
            memory: [0; 4096],
            keypad: Keypad::new(),
            display: Display::new(),
        }
    }

    pub fn init(&mut self, game: &str) {
        for i in 0..80 {
            self.memory[i] = FONTS[i];
        };

        let mut f = File::open(game).expect("Unable to locate ROM");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("Unable to read ROM data");

        for i in 0..buffer.len() {
            self.memory[i + self.cpu.program as usize] = buffer[i];
            self.cpu.program += 1;
        };
    }

    pub fn cycle(&mut self) {
        let opcode: u16 = (self.memory[self.cpu.program as usize] as u16) << 8
            | (self.memory[(self.cpu.program + 1) as usize] as u16);

        self.cpu.process(opcode, &mut self.display, &mut self.keypad, &mut self.memory);
    }
}