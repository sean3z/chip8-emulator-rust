use cpu::Cpu;

pub struct System {
    pub cpu: Cpu,
    pub memory: [u8; 4096],
    // pub keypad: Keypad,
    // pub display: Display
}

impl System {
    pub fn new() -> System {
        System {
            cpu: Cpu {
                pc: 0,
                stack: [0; 16]
            },
            memory: [0; 4096]
        }
    }

    pub fn init(&self) {
        println!("Hello, world!");
    }

    pub fn load(&self, game: &str) {
        println!("{}", game);
    }

    pub fn cycle(&self) {

    }

    pub fn draw(&self) {

    }
}