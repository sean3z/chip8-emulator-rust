pub struct Cpu {
    pc: u16, // program counter
    stack: [u16; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0x200,
            stack: [0; 16]
        }
    }

    pub fn process(&mut self, memory: [u8; 4096]) {
        let opcode: u16 = memory[self.pc as usize] as u16;
        self.pc += 2;
    }
}