use display::Display;
use keypad::Keypad;

pub struct Cpu {
    pub program: u16, // program counter
    index: u16, // index register

    stack: [u16; 16], // stack storage
    stack_pointer: u8, // stack pointer

    delay_timer: u8, // delay timer
    sound_timer: u8,  // sound timer

    v: [u8; 16], // cpu registers (V0 through Ee)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program: 0x200,
            index: 0,

            stack: [0; 16],
            stack_pointer: 0,

            delay_timer: 0,
            sound_timer: 0,

            v: [0; 16],
        }
    }

    pub fn process(&mut self, opcode: u16, display: &mut Display, keypad: &mut Keypad, memory: &mut [u8; 4096]) {
        // extract various opcode parameters
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        // break up into nibbles
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        println!("{}, {}, {}, {}", op_1, op_2, op_3, op_4);
        println!("program counter: {}", self.program);

        self.program += 2;

        match (op_1, op_2, op_3, op_4) {
            (0, 0, 0xE, 0) => display.clear(),
            (0, 0, 0xE, 0xE) => {
                self.stack_pointer = self.stack_pointer - 1;
                self.program = self.stack[self.stack_pointer as usize];
            },
            (0x1, _, _, _) => self.program = nnn,
            (0x2, _, _, _) => {
                self.stack[self.stack_pointer as usize] = self.program;
                self.stack_pointer = self.stack_pointer + 1;
                self.program = nnn;
            },
            (0x3, _, _, _) => self.program += if vx == kk { 2 } else { 0 },
            (0x4, _, _, _) => self.program += if vx != kk { 2 } else { 0 },
            (0x5, _, _, _) => self.program += if vx == vy { 2 } else { 0 },
            (0x6, _, _, _) => self.v[x] = kk,
            (0x7, _, _, _) => self.v[x] += kk,
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],
            (0x8, _, _, 0x1) => self.v[x] = self.v[x] | self.v[y],
            (0x8, _, _, 0x2) => self.v[x] = self.v[x] & self.v[y],
            (0x8, _, _, 0x3) => self.v[x] = self.v[x] ^ self.v[y],
            (0x8, _, _, 0x4) => {
                let res = self.v[x] as u16 + self.v[y] as u16;
                self.v[0xF] = if res > 0xFF { 1 } else { 0 };
                self.v[x] = (res & 0xFF) as u8;
            },
            (0x8, _, _, 0x5) => {
                let res = self.v[x] as i8 - self.v[y] as i8;
                self.v[x] = res as u8;
                self.v[0xF] = if res < 0 { 1 } else { 0 };
            },
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            },
            (0x8, _, _, 0x7) => {
                let res = self.v[y] as i8 - self.v[x] as i8;
                self.v[x] = res as u8;
                self.v[0xF] = if res < 0 { 1 } else { 0 };
            },
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            },
            (0x9, _, _, _) => self.program += if vx != vy { 2 } else { 0 },
            (0xA, _, _, _) => self.index = nnn,
            (0xB, _, _, _) => self.program = nnn + self.v[0] as u16,
            (0xC, _, _, _) => self.v[x] = 8 as u8 & kk, // RND
            (0xD, _, _, _) => {
                let collision = display.draw(vx as usize, vy as usize);
                self.v[0xF] = if collision { 1 } else { 0 };
            },
            (0xE, _, 0x9, 0xE) => self.program += if keypad.is_keydown(vx) { 2 } else { 0 },
            (0xE, _, 0xA, 0x1) => self.program += if keypad.is_keydown(vx) { 0 } else { 2 },
            (0xF, _, 0x0, 0x7) => self.v[x] = self.delay_timer,
            (0xF, _, 0x0, 0xA) => {
                self.program -= 2;
                /* for (i, key) in self.keypad.keys.iter().enumerate() {
                    if *key == true {
                        self.v[x] = i as u8;
                        self.pc +=2;
                    }
                } */
            },
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.v[x],
            (0xF, _, 0x1, 0xE) => self.index = self.index + self.v[x] as u16,
            (0xF, _, 0x2, 0x9) => self.index = vx as u16 * 5,
            (0xF, _, 0x3, 0x3) => {
                memory[self.index as usize] = vx / 100;
                memory[self.index as usize + 1] = (vx / 10) % 10;
                memory[self.index as usize + 2] = (vx % 100) % 10;
            },
            (0xF, _, 0x5, 0x5) => {
                memory[(self.index as usize)..(self.index + x as u16 + 1) as usize]
                    .copy_from_slice(&self.v[0..(x as usize + 1)]);
            },    
            (0xF, _, 0x6, 0x5) =>  {
                self.v[0..(x as usize + 1)]
                     .copy_from_slice(&memory[(self.index as usize)..(self.index + x as u16 + 1) as usize]);
            },
            (_, _, _, _) => println!("{}", "opcode unimplemented")
        }


    }
}