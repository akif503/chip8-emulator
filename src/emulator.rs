mod display;
mod instructions;

use display::Display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::time::Duration;

pub struct Emulator {
    cpu: CPU,
    memory: Vec<u8>,
    screen: Display,
    event_pump: EventPump,
    program_size: usize,
    running: bool,
    PROGRAM_START: usize
}

struct Timer {
    DT: u8,
    ST: u8
}

struct CPU {
    registers: [u8; 16],
    VI: u16,
    stack: [u16; 16],
    timer: Timer,
    PC: u16,
    SP: u8,
}


impl Emulator {
    pub fn new(width: u32, height: u32) -> Emulator {
        let cpu = CPU {
            registers: [0; 16],
            VI: 0,
            stack: [0; 16],
            timer: Timer {
                DT: 0,
                ST: 0
            },
            PC: 0,
            SP: 0,
        };

        // The chip-8 langauge is capable of accessing up to 4KB of RAM
        let mut memory: Vec<u8> = [0 ;4096].to_vec();

        // load fonts
        let fonts = [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                     0x20, 0x60, 0x20, 0x20, 0x70, // 1
                     0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                     0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                     0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                     0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                     0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                     0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                     0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                     0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                     0xF0, 0x90, 0x90, 0x90, 0x90, // A
                     0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                     0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                     0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                     0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                     0xF0, 0x80, 0xF0, 0x80, 0x80  // F
                    ];
        
        for addr in 0..fonts.len() {
            memory[addr] = fonts[addr];
        }

        let screen = Display::new(width, height);

        Emulator {
            cpu,
            event_pump: screen.sdl_context.event_pump().unwrap(),
            screen,
            memory,
            program_size: 0,
            running: false,
            PROGRAM_START: 0x200,
        }
    }

    pub fn read_rom(&mut self, program: &mut Vec<u8>) {

        let addr: usize = self.PROGRAM_START;
        self.program_size = program.len();
        
        for idx in 0..self.program_size {
            self.write(addr + idx, program[idx]);
        }
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        if addr == 15 {
            println!("{:#x}", value);
        }
        self.memory[addr] = value;
    }

    pub fn read(& self, addr: usize) -> u8 {
        return self.memory[addr]
    }

    fn convert_to_opcode(&self, b1: u8, b2: u8) -> u16 {
        let opcode: u16 = (b1 as u16) << 8 | b2 as u16;

        return opcode;
    }

    pub fn run(&mut self) {
        let mut instructions: Vec<u16> = Vec::new();
        
        // Convert to opcodes
        for idx in (0..(self.program_size - 1)).step_by(2) {
            let b1: u8 = self.read(self.PROGRAM_START + idx);
            let b2: u8 = self.read(self.program_size + idx + 1);

            let opcode = (b1 as u16) << 8 | b2 as u16;

            instructions.push(opcode);
        }            

        // for opcode in instructions.iter() {
        //     self.execute(*opcode);
        // }

        // let no_opcodes = instructions.len();
        // let mut opcode_idx = 0;

        self.running = true;
        self.cpu.PC = self.PROGRAM_START as u16;

        while self.running {
            // Handle Events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                    },
                    _ => {}
                }
            }

            // Update
            // if opcode_idx < no_opcodes {
            //     print!("Execute [{:#x}]: ", instructions[opcode_idx]);
            //     self.execute(instructions[opcode_idx]);
            // }

            // Reference: https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
            
            // Fetch
            let pc: usize = self.cpu.PC as usize;
            let opcode = self.convert_to_opcode(self.read(pc), self.read(pc + 1));
            self.cpu.PC = (self.cpu.PC + 2) % ((self.memory.len() - 1) as u16);

            print!("Execute [{:#x}]: ", opcode);

            // Decode & Execute
            self.execute(opcode);

            // Render Canvas
            self.screen.render();

            if self.cpu.timer.DT != 0 {
                self.cpu.timer.DT -= 1;
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn get_mapped_key(&self, primitive_key: u8) -> Keycode {
        match primitive_key {
            0 => {
                Keycode::Num0
            },
            1 => {
                Keycode::Num1
            },
            2 => {
                Keycode::Num2
            },
            3 => {
                Keycode::Num3
            },
            4 => {
                Keycode::Num4
            },
            5 => {
                Keycode::Num5
            },
            6 => {
                Keycode::Num6
            },
            7 => {
                Keycode::Num7
            },
            8 => {
                Keycode::Num8
            },
            9 => {
                Keycode::Num9
            },
            10 => {
                Keycode::A
            },
            11 => {
                Keycode::B
            },
            12 => {
                Keycode::C
            },
            13 => {
                Keycode::D
            },
            14 => {
                Keycode::E
            },
            15 => {
                Keycode::F
            },
            _ => {
                Keycode::Space
            }
        }
    }

}

// NNN: Address
// NN: 1 byte constant
// N: half byte

// X & Y: [0 - F]: Each identifying a 4-bit egister
//  - Registers are references as VX/VY.
// I: 2 byte register (For memory address) (Similar to void pointer)