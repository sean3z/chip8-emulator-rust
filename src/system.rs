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
    pub fn new(game: &str) -> System {
        let mut memory = [0; 4096];
        let cpu = Cpu::new();

        for i in 0..80 {
            memory[i] = FONTS[i];
        };

        let mut f = File::open(game).expect("Unable to locate ROM");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("Unable to read ROM data");

        for i in 0..buffer.len() {
            memory[i + cpu.program as usize] = buffer[i];
        };

        System {
            cpu: cpu,
            memory: memory,
            keypad: Keypad::new(),
            display: Display::new(),
        }
    }

    pub fn cycle(&mut self) {
        let opcode: u16 = (self.memory[self.cpu.program as usize] as u16) << 8
            | (self.memory[(self.cpu.program + 1) as usize] as u16);

        self.cpu.process(opcode, &mut self.display, &mut self.keypad, &mut self.memory);
    }
}