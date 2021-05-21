mod emulator;

use std::env;
use std::fs;
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
