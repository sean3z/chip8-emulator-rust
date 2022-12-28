#![allow(non_snake_case)]

use rand;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
// use std::slice::*;
// use rand::Rng;
use display::Display;
use keypad::Keypad;

pub struct Cpu {
    program: usize, // program counter starts at 512 bytes
    opcode: u16, // current opcode
    stack: [u16; 16], // stack storage
    stack_pointer: usize, // stack pointer
    delay_timer: u8, // delay timer
    sound_timer: u8, // sound timer
    v: [u8; 16], // cpu registers (V0 through Ee)
    i: usize, // index register
    memory: [u8; 4096], // system memory

    pub keypad: Keypad,
    pub display: Display,
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut memory = [0; 4096];

        // load fonts into memory
        for i in 0..80 {
            memory[i] = FONTS[i];
        };

        Cpu {
            opcode: 0,
            program: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            i: 0x200,
            memory: memory,
            keypad: Keypad::new(),
            display: Display::new(),
        }
    }

    pub fn load_game(&mut self, game: &str) {
        // attempt to load supplied ROM
        let mut reader = File::open(game).expect("Unable to locate ROM");
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("Unable to read ROM data");

        // load ROM into memory (AFTER system reserved memory)
        for i in 0..buffer.len() {
            self.memory[i + self.program] = buffer[i];
        };
    }

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        self.execute_opcode();

        if self.delay_timer > 0 { self.delay_timer -= 1; }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 { println!("BEEP!\n"); }
            self.sound_timer -= 1;
        }

        thread::sleep(time::Duration::from_micros(500));
    }

    fn fetch_opcode(&mut self) {
        self.opcode = (self.memory[self.program as usize] as u16) << 8 | (self.memory[(self.program + 1) as usize] as u16);
    }

    fn execute_opcode(&mut self) {
        match self.opcode & 0xf000 {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xxx(),
            0xA000 => self.op_Axxx(),
            0xB000 => self.op_Bxxx(),
            0xC000 => self.op_Cxxx(),
            0xD000 => self.op_Dxxx(),
            0xE000 => self.op_Exxx(),
            0xF000 => self.op_Fxxx(),
            _ => not_implemented(self.opcode as usize, self.program)
        }
    }

    fn op_0xxx(&mut self) {
        match self.opcode & 0x000F {
            0x0000 => { self.display.clear() }
            0x000E => {
                self.stack_pointer -= 1;
                self.program = self.stack[self.stack_pointer] as usize;
            }
            _ => { not_implemented(self.opcode as usize, self.program) }
        }
        self.program += 2;
    }

    // Jumps to address
    fn op_1xxx(&mut self) { self.program = self.op_nnn() as usize; }

    // Calls subroutine
    fn op_2xxx(&mut self) {
        self.stack[self.stack_pointer] = self.program as u16;
        self.stack_pointer += 1;
        self.program = self.op_nnn() as usize;
    }

     // Skips the next instruction if VX equals NN
     fn op_3xxx(&mut self) {
        self.program += if self.v[self.op_x()] == self.op_nn() { 4 } else { 2 }
    }

    // Skips the next instruction if VX doesn't equal NN
    fn op_4xxx(&mut self) {
        self.program += if self.v[self.op_x()] != self.op_nn() { 4 } else { 2 }
    }

    // Skips the next instruction if VX equals VY
    fn op_5xxx(&mut self) {
        self.program += if self.v[self.op_x()] == self.v[self.op_y()] { 4 } else { 2 }
    }

    // Sets VX to NN
    fn op_6xxx(&mut self) {
        self.v[self.op_x()] = self.op_nn();
        self.program += 2;
    }

    // Adds NN to VX
    fn op_7xxx(&mut self) {
        self.v[self.op_x()] += self.op_nn();
        self.program += 2;
    }

    fn op_8xxx(&mut self) {
        match self.opcode & 0x000F {
            0 => { self.v[self.op_x()] = self.v[self.op_y()]; }
            1 => { self.v[self.op_x()] |= self.v[self.op_y()]; }
            2 => { self.v[self.op_x()] &= self.v[self.op_y()]; }
            3 => { self.v[self.op_x()] ^= self.v[self.op_y()]; }
            4 => {
                let (res, overflow) = self.v[self.op_x()].overflowing_add(self.v[self.op_y()]);
                match overflow {
                    true => self.v[0xF] = 1,
                    false => self.v[0xF] = 0,
                }
                self.v[self.op_x()] = res; { 0 };
            }
            5 => {
                let (res, overflow) = self.v[self.op_x()].overflowing_sub(self.v[self.op_y()]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[self.op_x()] = res;
            }
            6 => {
                self.v[0xF] = self.v[self.op_x()] & 0x1;
                self.v[self.op_x()] >>= 1;
            }
            7 => {
                let (res, overflow) = self.v[self.op_y()].overflowing_sub(self.v[self.op_x()]);
                match overflow {
                    true => self.v[0xF] = 0,
                    false => self.v[0xF] = 1,
                }
                self.v[self.op_x()] = res;
            }
            0xE => {
                // self.v[0xF] = self.v[self.op_x()] >> 7;
                self.v[0xF] = self.v[self.op_x()] & 0x80;
                self.v[self.op_x()] <<= 1;
            }
            _ => not_implemented(self.opcode as usize, self.program)
        }
        self.program += 2;
    }

    fn op_9xxx(&mut self) {
        self.program += if self.v[self.op_x()] != self.v[self.op_y()] { 4 } else { 2 }
    }

    fn op_Axxx(&mut self) {
        self.i = self.op_nnn() as usize;
        self.program += 2;
    }

    fn op_Bxxx(&mut self) { self.program = (self.op_nnn() + (self.v[0] as u16)) as usize; }

    fn op_Cxxx(&mut self) {
        /*
        let mut rng = rand::thread_rng();
        self.v[self.op_x()] = rng.gen::<u8>() & kk
        */
        self.v[self.op_x()] = self.op_nn() & rand::random::<u8>();
        self.program += 2;
    }

    fn op_Dxxx(&mut self) {
        let from = self.i;
        let to = from + (self.op_n() as usize);
        let x = self.v[self.op_x()];
        let y = self.v[self.op_y()];

        self.v[0xF] = self.display.draw(x as usize, y as usize, &self.memory[from..to]);
        self.program += 2;
    }

    fn op_Exxx(&mut self) {
        let v = self.v[self.op_x()] as usize;
        self.program += match self.opcode & 0x00FF {
            0x9E => if self.keypad.pressed(v) { 4 } else { 2 },
            0xA1 => if !self.keypad.pressed(v) { 4 } else { 2 },
            _ => 2
        }
    }

    fn op_Fxxx(&mut self) {
        match self.opcode & 0x00FF {
            0x07 => { self.v[self.op_x()] = self.delay_timer; }
            0x0A => { self.wait_keypress(); }
            0x15 => { self.delay_timer = self.v[self.op_x()]; }
            0x18 => { self.sound_timer = self.v[self.op_x()]; }
            0x1E => { self.i += self.v[self.op_x()] as usize; }
            0x29 => { self.i = (self.v[self.op_x()] as usize) * 5; }
            0x33 => {
                self.memory[self.i] = self.v[self.op_x()] / 100;
                self.memory[self.i + 1] = (self.v[self.op_x()] / 10) % 10;
                self.memory[self.i + 2] = (self.v[self.op_x()] % 100) % 10;
            }
            0x55 => {
                for i in 0..(self.op_x() + 1) {
                    self.memory[self.i + i] = self.v[i]
                }
                self.i += self.op_x() + 1;
            }
            0x65 => {
                for i in 0..(self.op_x() + 1) {
                    self.v[i] = self.memory[self.i + i]
                }
                self.i += self.op_x() + 1;
            }
            _ => { not_implemented(self.opcode as usize, self.program) }
        }
        self.program += 2;
    }


    fn op_x(&self)   -> usize { ((self.opcode & 0x0F00) >> 8) as usize }
    fn op_y(&self)   -> usize { ((self.opcode & 0x00F0) >> 4) as usize }
    fn op_n(&self)   -> u8 { (self.opcode & 0x000F) as u8 }
    fn op_nn(&self)  -> u8 { (self.opcode & 0x00FF) as u8 }
    fn op_nnn(&self) -> u16 { self.opcode & 0x0FFF }

    fn wait_keypress(&mut self) {
        for i in 0u8..16 {
            if self.keypad.pressed(i as usize) {
                self.v[self.op_x()] = i;
                break;
            }
        }
        self.program -= 2;
    }
}

fn not_implemented(op: usize, pc: usize) { println!("Not implemented:: op: {:x}, pc: {:x}", op, pc) }

static FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];