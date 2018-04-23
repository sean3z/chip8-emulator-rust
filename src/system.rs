use display::FONTS;
use cpu::Cpu;

pub struct System {
    cpu: Cpu,
    memory: [u8; 4096],
    // pub keypad: Keypad,
    // pub display: Display
}

impl System {
    pub fn new() -> System {
        System {
            cpu: Cpu::new(),
            memory: [0; 4096]
        }
    }

    pub fn init(&mut self, game: &str) {
        for i in 0..80 {
            self.memory[i] = FONTS[i];
        }

        println!("{}", game);
    }

    pub fn cycle(&mut self) {
       self.cpu.process(self.memory);
    }

    pub fn draw(&self) {

    }
}