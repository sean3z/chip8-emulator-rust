pub struct Cpu {
    pub program: u16, // program counter
    index: u16, // index counter

    stack: [u16; 16], // stack storage
    pointer: u8, // stack pointer

    delay: u8 // delay timer
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program: 0x200,
            index: 0,

            stack: [0; 16],
            pointer: 0,

            delay: 0
        }
    }

    pub fn process(&mut self, memory: [u8; 4096]) {
        let opcode: u16 = (memory[self.program as usize] as u16) << 8 
            | (memory[(self.program + 1) as usize] as u16);

        self.program += 2;

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        match (op_1, op_2, op_3, op_4) {
            (_, _, _, _) => {
                println!("{}, {}, {}, {}", op_1, op_2, op_3, op_4);
            }
        }

        println!("{:?}", opcode);
        println!("{:?}", self.program);
    }
}