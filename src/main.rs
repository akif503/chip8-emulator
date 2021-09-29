mod emulator;

use std::env;
use std::fs;
use sdl2::event::Event;
use std::time::Duration;
use sdl2::keyboard::Keycode;
use emulator::Emulator;

// For simplicity we'll assume width and height are multiples of our final mapping
const WIDTH: u32 = 768;
const HEIGHT: u32 = 576;

fn main() {
    // Read file 
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <filename>");
        return
    }

    let file_path = &args[1];
    let mut program: Vec<u8> = fs::read(file_path).expect("Cannot read the file");

    // let mut screen = Display::new(WIDTH, HEIGHT);
    let mut emulator = Emulator::new(WIDTH, HEIGHT);

    emulator.read_rom(&mut program);
    emulator.run();
}

impl Emulator {
    fn run(&mut self) {
        let mut instructions: Vec<u16> = Vec::new();

        // Convert to opcodes
        for idx in (0..(self.program_size - 1)).step_by(2) {
            let b1: u8 = self.read(self.PROGRAM_START + idx);
            let b2: u8 = self.read(self.program_size + idx + 1);

            let opcode = (b1 as u16) << 8 | b2 as u16;

            instructions.push(opcode);
        }            

        self.running = true;
        self.cpu.PC = self.PROGRAM_START as u16;

        let mut count = 0;

        while self.running {
            let mut key_event_triggered: Vec<Event> = Vec::new();

            // Handle Events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        self.running = false;
                    },
                    Event::KeyDown {..} => {
                        //println!("{:#?}", event);
                        key_event_triggered.push(event);
                    }
                    _ => {
                    }
                }
            }

            // Fetch
            let pc: usize = self.cpu.PC as usize;
            let opcode = self.convert_to_opcode(self.read(pc), self.read(pc + 1));
            self.cpu.PC = (self.cpu.PC + 2) % ((self.memory.len() - 1) as u16);

            print!("Execute [{:#x}]: ", opcode);

            // Decode & Execute
            self.execute(opcode, key_event_triggered);

            // Render Canvas
            self.screen.render();

            if self.cpu.timer.DT != 0 {
                // count += 1;

                //if count % 30 == 0 {
                    self.cpu.timer.DT -= 1;
                // }
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
