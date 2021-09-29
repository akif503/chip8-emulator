use crate::emulator::Emulator;

use sdl2::event::Event;
use rand::{Rng, thread_rng};

fn hex_to_int(hex_str: &str) -> u16 {
    return u16::from_str_radix(hex_str, 16).expect("Cannot convert hex.");
}

impl Emulator {
    pub fn execute(&mut self, opcode: u16, key_event_triggered: Vec<Event>) {
        // This will be an array of bytes
        let temp = format!("{:#06x}", opcode);
        let opcode_str = temp.strip_prefix("0x").unwrap();
        // let opcode_bytes = opcode_str.strip_prefix("0x").unwrap().as_bytes();
        // Big endian
        
        // Values
        let addr: u16 = hex_to_int(&opcode_str[1..]);
        let byte: u8 = hex_to_int(&opcode_str[2..]) as u8;
        let nibble: u8 = hex_to_int(&opcode_str[3..]) as u8;

        // Registers
        let vx_addr: usize = hex_to_int(&opcode_str[1..2]) as usize;
        let vy_addr: usize = hex_to_int(&opcode_str[2..3]) as usize;
        let vf_addr = 15;
        let vx = self.cpu.registers[vx_addr];
        let vy = self.cpu.registers[vy_addr];
        let vi = self.cpu.VI;

        match &opcode_str[0..1] {
            "0" => {
                match opcode {
                    0x00E0 => {
                        // CLS
                        println!("CLS");
                        self.screen.clear();
                    },
                    0x00EE => {
                        // RET
                        self.cpu.PC = self.cpu.stack[self.cpu.SP as usize] as u16;
                        if self.cpu.SP > 0 {
                            self.cpu.SP -= 1;
                        } 
                        else {
                            println!("Error: Invalid Return");
                        }
                    },
                    _ => {
                        // SYS
                    }
                }
            },
            "1" => {
                // JP addr
                println!("JP {:#x}", addr);
                self.cpu.PC = addr;
            },
            "2" => {
                // CALL addr
                println!("CALL {:#x}", addr);

                self.cpu.SP += 1;
                self.cpu.stack[self.cpu.SP as usize] = self.cpu.PC;
                self.cpu.PC = addr;
            },
            "3" => {
                // SE Vx, byte
                println!("SE {:#x} {:#x}", vx, byte);
                if vx == byte {
                    self.cpu.PC += 2;
                }
            },
            "4" => {
                // SNE Vx, byte
                println!("SNE {:#x} {:#x}", vx, byte);

                if vx != byte {
                    self.cpu.PC += 2;
                }
            },
            "5" => {
                match &opcode_str[3..] {
                    "0" => {
                        // SE Vx, Vy
                        println!("SE {:#x} {:#x}", vx, vy);
                        if vx == vy {
                            self.cpu.PC += 2;
                        }
                    },
                    _ => {
                        // Error
                        println!("Unknown opcode {:#x}", opcode);
                    }
                }
            },
            "6" => {
                // LD Vx, byte
                println!("LD {:#x} {:#x}", vx, byte);
                self.cpu.registers[vx_addr] = byte;
            },
            "7" => {
                // ADD Vx, byte
                println!("Add {:#x} {:#x}", vx, byte);

                // Measure against overflow
                let temp: u16 = vx as u16 + byte as u16;
                if temp >= (1 << 8) {
                    self.cpu.registers[vx_addr] = temp as u8;
                }
                else {
                    // Here's the actual instruction
                    self.cpu.registers[vx_addr] += byte;
                }
            },
            "8" => {
                match &opcode_str[3..] {
                    "0" => {
                        // LD Vx, Vy
                        println!("LD {:#x} {:#x}", vx, vy);
                        self.cpu.registers[vx_addr] = vy;
                    },
                    "1" => {
                        // OR Vx, Vy
                        println!("OR {:#x} {:#x}", vx, vy);
                        self.cpu.registers[vx_addr] = vx | vy;
                    },
                    "2" => {
                        // AND Vx, Vy
                        println!("AND {:#x} {:#x}", vx, vy);
                        self.cpu.registers[vx_addr] = vx & vy;
                    },
                    "3" => {
                        // XOR Vx, Vy
                        println!("XOR {:#x} {:#x}", vx, vy);
                        self.cpu.registers[vx_addr] = vx ^ vy;
                    },
                    "4" => {
                        // ADD Vx, Vy, & carry
                        println!("ADD {:#x} {:#x}", vx, vy);
                        self.cpu.registers[vx_addr] = ((vx as u16) + (vy as u16)) as u8;
                        self.cpu.registers[vf_addr] = 0;

                        if ((vx as u16) + (vy as u16)) > 255 {
                            self.cpu.registers[vf_addr] = 1;
                        }
                    },
                    "5" => {
                        // SUBN Vx, Vy, carry
                        println!("SUB {:#x} {:#x}", vx, vy);
                        if vx > vy {
                            self.cpu.registers[vf_addr] = 1;
                            self.cpu.registers[vx_addr] = vx - vy;
                        }
                        else {
                            self.cpu.registers[vf_addr] = 0;
                            self.cpu.registers[vx_addr] = vy - vx;
                        }
                    },
                    "6" => {
                        // SHR Vx, Vy
                        println!("SHR {:#x} {:#x}", vx, vy);

                        // If LSB is 1, then set VF to 1 otherwise 0
                        if vx % 2 == 1 {
                            self.cpu.registers[vf_addr] = 1;
                        } 
                        else {
                            self.cpu.registers[vf_addr] = 0;
                        }

                        // Then divide vx by 2
                        self.cpu.registers[vx_addr] = vx >> 1;
                    },
                    "7" => {
                        // SUBN Vx, Vy, carry
                        println!("SUBN {:#x} {:#x}", vx, vy);

                        if vy > vx {
                            self.cpu.registers[vf_addr] = 1;
                            self.cpu.registers[vx_addr] = vy - vx;
                        }
                        else {
                            self.cpu.registers[vf_addr] = 0;
                            self.cpu.registers[vx_addr] = vx - vy;
                        }
                    },
                    "E" | "e" => {
                        // SHL Vx, Vy
                        println!("SHL {:#x} {:#x}", vx, vy);

                        // IF MSB of Vx is 1, then set VF to 1 otherwise 0
                        let check_msb = vx & 128;

                        if check_msb != 0 {
                            self.cpu.registers[vf_addr] = 1;
                        }
                        else {
                            self.cpu.registers[vf_addr] = 0;
                        }

                        // Then multiply vx by 2
                        self.cpu.registers[vx_addr] = vx << 1;
                    },
                    _ => {
                        // Error
                        println!("Unknown opcode {:#x}", opcode);
                    }
                }
            },
            "9" => {
                match &opcode_str[3..] {
                    "0" => {
                        // SNE Vx, Vy
                        println!("SNE {:#x} {:#x}", vx, vy);

                        if vx != vy {
                            self.cpu.PC += 2;
                        }
                    },
                    _ => {
                        // Error
                        println!("Unknown opcode {:#x}", opcode);
                    }
                }
            },
            "a" | "A" => {
                // LD I, addr
                println!("LD {:#x} {:#x}", self.cpu.VI, addr);

                self.cpu.VI = addr;
            },
            "b"| "B" => {
                // JP V0, addr
                println!("JP {:#x} {:#x}", self.cpu.registers[0], addr);

                self.cpu.PC = addr + (self.cpu.registers[0] as u16);
            },
            "c" | "C" => {
                // RND Vx, byte
                println!("RND {:#x} {:#x}", vx, byte);

                let val = thread_rng().gen::<u8>();
                
                self.cpu.registers[vx_addr] = val & byte;

                // println!("{}", val & byte);
            },
            "d" | "D" => {
                // DRW Vx, Vy, nibble
                println!("DRW {:#x} {:#x} {:#x}", vx, vy, nibble);
                let vf = self.screen.xor_sprite(&self.memory, vi as usize, (vx, vy), nibble);
                self.cpu.registers[vf_addr] = vf;
            },
            "e" | "E" => {
                match &opcode_str[2..] {
                    "9E" | "9e" => {
                        // SKP Vx
                        // Skip next instruction if key with the value of Vx is pressed.
                        println!("SKP {:#x}", vx);

                        let desired_key = self.get_mapped_key(vx);
                        println!("keys: {}", desired_key);
                        for event in key_event_triggered.iter() {
                            match event {
                                Event::KeyDown { keycode: Some(key), ..} => {
                                    println!("keys: {} {}", desired_key, key);
                                    if desired_key == *key {
                                        self.cpu.PC += 2;
                                    }
                                },
                                _ => {}
                            }
                        }
                    },
                    "A1" | "a1" => {
                        // SKNP Vx
                        println!("SKNP {:#x}", vx);

                        let desired_key = self.get_mapped_key(vx);
                        println!("keys: {}", desired_key);
                        for event in key_event_triggered.iter() {
                            match event {
                                Event::KeyDown { keycode: Some(key), ..} => {
                                    // println!("HURRAH!!");
                                    if desired_key != *key {
                                        self.cpu.PC += 2;
                                    }
                                },
                                _ => {println!("{:#?}", event)}
                            }
                        }
                    },
                    _ => {
                        // Error
                        println!("Unknown opcode {:#x}", opcode);
                    }
                }
            },
            "f" | "F" => {
                match &opcode_str[2..] {
                    "07" => {
                        // LD Vx, DT
                        println!("LD {:#x} {:#x}", vx, self.cpu.timer.DT);
                        self.cpu.registers[vx_addr] = self.cpu.timer.DT;
                        println!("DT: {}", self.cpu.timer.DT);
                    },
                    "0a" | "0A" => {
                        // LD Vx, K
                        println!("LD {:#x} {:#x}", vx, 0x100);

                        let mut key: u8 = 16;

                        'outer: while key == 16 {
                            // Wait for a keypress
                            self.event_pump.wait_event();

                            let event = self.event_pump.poll_event();

                            if event != None {
                                match event.unwrap() {
                                    Event::Quit {..} => {
                                        // Trigger exit ------
                                        self.running = false;
                                        break 'outer;
                                    },
                                    Event::KeyDown {keycode: Some(keycode), ..} => {
                                        // let keycode = keycode.unwrap();
                                        println!("{}", keycode);

                                        break 'outer;
                                    },
                                    Event::TextInput {text, ..} => {
                                        println!("k: {}", text);
                                        
                                        if "0123456789".contains(&text) {
                                            key = text.parse::<u8>().expect("Wrong key");
                                        }
                                        else if (text == "A") | (text == "a") {
                                            key = 10;
                                        }
                                        else if (text == "B") | (text == "b") {
                                            key = 11;
                                        }
                                        else if (text == "C") | (text == "c") {
                                            key = 12;
                                        }
                                        else if (text == "D") | (text == "d") {
                                            key = 13;
                                        }
                                        else if (text == "E") | (text == "e") {
                                            key = 14; 
                                        }
                                        else if (text == "F") | (text == "f") {
                                            key = 15; 
                                        }
                                        else {
                                            continue;
                                        }

                                        break 'outer;
                                    }
                                    _ => {
                                        //println!("{:#?}", event);
                                    }
                                }
                            }
                            
                        }

                        println!("{}", key);
                        self.cpu.registers[vx_addr] = key;
                    },
                    "15" => {
                        // LD DT, Vx
                        println!("LD {:#x} {:#x}", self.cpu.timer.DT, vx);
                        self.cpu.timer.DT = vx;
                        println!("DT: {}", self.cpu.timer.DT);
                    },
                    "18" => {
                        // LD ST, Vx
                        println!("LD {:#x} {:#x}", self.cpu.timer.ST, vx);
                        self.cpu.timer.ST = vx;
                    },
                    "1e" | "1E" => {
                        // ADD I, Vx
                        println!("Add {:#x} {:#x}", self.cpu.VI, vx);
                        self.cpu.VI += vx as u16;
                    },
                    "29" => {
                        // LD F, Vx
                        // Set I = location of sprite for digit Vx.
                        println!("LD {:#x} {:#x}", vx * 5, vx);

                        // vx * 5 corresponds to the index of the fonts array
                        // Since, each font has a length of 5 bytes, so each 
                        // font is 5 bytes apart
                        self.cpu.VI = (vx * 5) as u16;

                        // println!("font: {}", self.cpu.VI);
                        // println!("{} - {} - {}", self.cpu.registers[0], self.cpu.registers[1], self.cpu.registers[2]);
                    },
                    "33" => {
                        // LD B, Vx
                        println!("LD B {:#x}", vx_addr);
                        // Get the value at that is stored in Vx
                        let mut value: u8 = vx;
                        println!("{}", value);

                        // Store value in regiser I, I+1, I+2
                        for idx in 0..3 {
                            let digit = value % 10;
                            
                            self.write((vi + 2 - idx) as usize, digit);

                            value = value / 10; 
                        }
                    },
                    "55" => {
                        // LD [I], Vx
                        println!("LD [I] {:#x}", vx);

                        for idx in 0..vx_addr {
                            self.write(vi as usize + idx, self.cpu.registers[idx]);
                        }
                    },
                    "65" => {
                        // LD Vx, [I]
                        println!("LD {:#x} [I]", vx_addr);

                        for idx in 0..(vx_addr+1) {
                            self.cpu.registers[idx] = self.read(vi as usize + idx);
                        }
                    },
                    _ => {
                        // Error
                        println!("Unknown opcode {:#x}", opcode);
                    }
                }
            }
            _ => {
                println!("Unknown opcode {:#x}", opcode);
            }
        }
    }
}