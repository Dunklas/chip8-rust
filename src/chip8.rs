use std::num::Wrapping;

pub fn new(rom_bytes: &[u8]) -> Chip8 {
    let mut chip8 = Chip8 {
        op_code: 0,
        memory: [0; 4096],
        v: [0; 16],
        index: 0,
        program_counter: 0x200,
        gfx: [0; 64 * 32],
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; 16],
        stack_pointer: 0,
        key: [0; 16],
        draw: false
    };

    let font_set = font_set();
    for (i, _byte) in font_set.iter().enumerate() {
        // if i % 5 == 0 {
        //     print!("\n");
        // }
        // println!("{:#010b}", font_set[i]);
        chip8.memory[i] = font_set[i];
    }

    for (i, &byte) in rom_bytes.iter().enumerate() {
        chip8.memory[i + 0x200] = byte;
    }

    return chip8;
}

fn font_set() -> [u8; 80] {
    [
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
    ]
}

pub struct Chip8 {
    op_code: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    index: u16,
    program_counter: u16,
    pub gfx: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: u16,
    key: [u8; 16],
    pub draw: bool
}

impl Chip8 {

    pub fn emulate_cycle(&mut self) {
        self.fetch_opcode();
        let op_code = self.op_code;
        Chip8::print_debug(&format!("OP: {:#06x}", self.op_code));
        match self.op_code & 0xF000 {
            0x0000 => {
                match op_code & 0x000F {
                    0x0000 => {
                        Chip8::print_debug(&format!("0x00E0: Clear screen"));
                        self.gfx = [0; 64 * 32];
                        self.draw = true;
                        self.program_counter += 2;
                    },
                    0x000E => {
                        Chip8::print_debug(&format!("0x00EE: Return from subroutine"));
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.program_counter += 2;
                    },
                    _ => {
                        Chip8::print_debug(&format!("Unrecognized op code: {:X?}", op_code));
                        return;
                    }
                }
            },
            0x1000 => {
                Chip8::print_debug(&format!("0x1NNN: Jumps to address NNN"));
                self.program_counter = op_code & 0x0FFF;
            },
            0x2000 => {
                Chip8::print_debug(&format!("0x2NNN: Calls subroutine at NNN"));
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = op_code & 0x0FFF;
            },
            0x3000 => {
                Chip8::print_debug(&format!("0x3XNN: Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)"));
                if self.v[((op_code & 0x0F00) >> 8) as usize] == (op_code & 0x00FF) as u8 {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            0x4000 => {
                Chip8::print_debug(&format!("0x4XNN: Skips the next instruction if VX doesn't equal NN"));
                if self.v[((op_code & 0x0F00) >> 8) as usize] != (op_code & 0x00FF) as u8 {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            },
            0x5000 => {
                Chip8::print_debug(&format!("0x5XY0: Skips the next instruction if VX equals VY"));
                if self.v[((op_code & 0x0F00) >> 8) as usize] == self.v[((op_code & 0x00F0) >> 4) as usize] {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x6000 => {
                Chip8::print_debug(&format!("0x6XNN: Sets VX to NN"));
                self.v[((op_code & 0x0F00) >> 8) as usize] = (op_code & 0x00FF) as u8;
                self.program_counter += 2;
            },
            0x7000 => {
                Chip8::print_debug(&format!("0x7XNN: Adds NN to VX. (Carry flag is not changed)"));
                let vx = Wrapping(self.v[((op_code & 0x0F00) >> 8) as usize]);
                let nn = Wrapping((op_code & 0x00FF) as u8);
                self.v[((op_code & 0x0F00) >> 8) as usize] = (vx + nn).0;
                self.program_counter += 2;
            },
            0x8000 => {
                match op_code & 0x000F {
                    0x0000 => {
                        Chip8::print_debug(&format!("0x8XY0: Sets VX to the value of VY"));
                        self.v[((op_code & 0x0F00) >> 8) as usize] = self.v[((op_code & 0x00F0) >> 4) as usize];
                        self.program_counter += 2;
                    },
                    0x0001 => {
                        Chip8::print_debug(&format!("0x8XY1: Sets VX to VX or VY. (Bitwise OR operation)"));
                        self.v[((op_code & 0x0F00) >> 8) as usize] = self.v[((op_code & 0x0F00) >> 8) as usize] | self.v[((op_code & 0x00F0) >> 4) as usize];
                        self.program_counter += 2;
                    }
                    0x0002 => {
                        Chip8::print_debug(&format!("0x8XY2: Sets VX to VX and VY. (Bitwise AND operation)"));
                        self.v[((op_code & 0x0F00) >> 8) as usize] = self.v[((op_code & 0x0F00) >> 8) as usize] & self.v[((op_code & 0x00F0) >> 4) as usize];
                        self.program_counter += 2;
                    },
                    0x0003 => {
                        Chip8::print_debug(&format!("0x8XY3: Sets VX to VX xor VY"));
                        self.v[((op_code & 0x0F00) >> 8) as usize] = self.v[((op_code & 0x0F00) >> 8) as usize] ^ self.v[((op_code & 0x00F0) >> 4) as usize];
                        self.program_counter += 2;
                    },
                    0x0004 => {
                        Chip8::print_debug(&format!("0x8XY4: Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't"));
                        if self.v[((op_code & 0x00F0) >> 4) as usize] > (0xFF - self.v[((op_code & 0x0F00) >> 8) as usize]) {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        let vx = Wrapping(self.v[((op_code & 0x0F00) >> 8) as usize]);
                        let vy = Wrapping(self.v[((op_code & 0x00F0) >> 4) as usize]);
                        self.v[((op_code & 0x0F00) >> 8) as usize] = (vx + vy).0;
                        self.program_counter += 2;
                    },
                    0x0005 => {
                        Chip8::print_debug(&format!("0x8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't"));
                        if self.v[((op_code & 0x0F00) >> 8) as usize] < self.v[((op_code & 0x00F0) >> 4) as usize] {
                            self.v[0xF] = 0;
                        } else {
                            self.v[0xF] = 1;
                        }
                        let vx = Wrapping(self.v[((op_code & 0x0F00) >> 8) as usize]);
                        let vy = Wrapping(self.v[((op_code & 0x00F0) >> 4) as usize]);
                        self.v[((op_code & 0x0F00) >> 8) as usize] = (vx - vy).0;
                        self.program_counter += 2;
                    },
                    0x0006 => {
                        Chip8::print_debug(&format!("0x8XY6: Stores the least significant bit of VX in VF and then shifts VX to the right by 1"));
                        self.v[0xF] = self.v[((op_code & 0x0F00) >> 8) as usize] & 0x1;
                        self.v[((op_code & 0x0F00) >> 8) as usize] >>= 1;
                        self.program_counter += 2;
                    }
                    0x0007 => {
                        Chip8::print_debug(&format!("0x8XY7: Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't"));
                        if self.v[((op_code & 0x00F0) >> 4) as usize] < self.v[((op_code & 0x0F00) >> 8) as usize] {
                            self.v[0xF] = 0;
                        } else {
                            self.v[0xF] = 1;
                        }
                        let vy = Wrapping(self.v[((op_code & 0x00F0) >> 4) as usize]);
                        let vx = Wrapping(self.v[((op_code & 0x0F00) >> 8) as usize]);
                        self.v[((op_code & 0x0F00) >> 8) as usize] = (vy - vx).0;
                        self.program_counter += 2;
                    }
                    0x000E => {
                        Chip8::print_debug(&format!("0x8XYE: Stores the most significant bit of VX in VF and then shifts VX to the left by 1"));
                        self.v[0xF] = self.v[((op_code & 0x0F00) >> 8) as usize] >> 7;
                        self.v[((op_code & 0x0F00) >> 8) as usize] <<= 1;
                        self.program_counter += 2;
                    }
                    _ => {
                        Chip8::print_debug(&format!("Unrecognized op code: {:X?}", op_code));
                        return;
                    }
                }
            }
            0x9000 => {
                Chip8::print_debug(&format!("0x9XY0: Skips the next instruction if VX doesn't equal VY"));
                if self.v[((op_code & 0x0F00) >> 8) as usize] != self.v[((op_code & 0x00F0) >> 4) as usize] {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0xA000 => {
                Chip8::print_debug(&format!("0xANNN: Sets I to the address NNN"));
                self.index = op_code & 0x0FFF;
                self.program_counter += 2;
            },
            0xB000 => {
                Chip8::print_debug(&format!("0xBNNN: Jumps to the address NNN plus V0"));
                self.program_counter = ((self.v[0x0] as u16) + (op_code & 0x0FFF));
            }
            0xC000 => {
                Chip8::print_debug(&format!("0xCXNN: Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN"));
                use rand::Rng;
                let mut rng = rand::thread_rng();
                self.v[((op_code & 0x0F00) >> 8) as usize] = rng.gen::<u8>() & ((op_code & 0x00FF) as u8);
                self.program_counter += 2;
            }
            0xD000 => {
                Chip8::print_debug(&format!("0xDXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels"));
                let x = self.v[((op_code & 0x0F00) >> 8) as usize] as u16;
                let y = self.v[((op_code & 0x00F0) >> 4) as usize] as u16;
                let height = op_code & 0x000F;

                self.v[0xF] = 0;
                for y_line in 0..height {
                    let pixel = self.memory[(self.index + y_line) as usize];
                    for x_line in 0..8 {
                        if (pixel & (0x80 >> x_line)) != 0 {
                            if self.gfx[((x + x_line + ((y + y_line) * 64)) % (64 * 32)) as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[((x + x_line + ((y + y_line) * 64)) % (64 * 32)) as usize] ^= 1;
                        }
                    }
                }

                self.draw = true;
                self.program_counter += 2;
            },
            0xE000 => {
                match op_code & 0x00FF {
                    0x009E => {
                        Chip8::print_debug(&format!("0xEX9E: Skips the next instruction if the key stored in VX is pressed"));
                        if self.key[self.v[((op_code & 0x0F00) >> 8) as usize] as usize] != 0 {
                            self.program_counter += 4;
                        } else {
                            self.program_counter += 2;
                        }
                    },
                    0x00A1 => {
                        Chip8::print_debug(&format!("0xEXA1: Skips the next instruction if the key stored in VX isn't pressed"));
                        if self.key[self.v[((op_code & 0x0F00) >> 8) as usize] as usize] == 0 {
                            self.program_counter += 4;
                        } else {
                            self.program_counter += 2;
                        }
                    },
                    _ => {
                        Chip8::print_debug(&format!("Unrecognized op code: {:X?}", op_code));
                        return;
                    }
                }
            }
            0xF000 => {
                match op_code & 0x00FF {
                    0x0007 => {
                        Chip8::print_debug(&format!("0xFX07: Sets VX to the value of the delay timer"));
                        self.v[((op_code & 0x0F00) >> 8) as usize] = self.delay_timer;
                        self.program_counter += 2;
                    }
                    0x0015 => {
                        Chip8::print_debug(&format!("0xFX15: Sets the delay timer to VX"));
                        self.delay_timer = self.v[((op_code & 0x0F00) >> 8) as usize];
                        self.program_counter += 2;
                    },
                    0x001E => {
                        Chip8::print_debug(&format!("0xFX1E: Adds VX to I. VF is set to 1 when there is a range overflow (I+VX>0xFFF), and to 0 when there isn't"));
                        if (self.v[((op_code & 0x0F00) >> 8) as usize]) as u16 > (0xFFF - self.index) {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.index += self.v[((op_code & 0x0F00) >> 8) as usize] as u16;
                        self.program_counter += 2;
                    },
                    0x0018 => {
                        Chip8::print_debug(&format!("0xFX18: Sets the sound timer to VX"));
                        self.sound_timer = self.v[((op_code & 0x0F00) >> 8) as usize];
                        self.program_counter += 2;
                    }
                    0x0029 => {
                        Chip8::print_debug(&format!("0xFX29: Sets I to the location of the sprite for the character in VX"));
                        let character = self.v[((op_code & 0x0F00) >> 8) as usize];
                        self.index = (character * 5).into(); // Each char takes 5 bytes
                        self.program_counter += 2;
                    },
                    0x0033 => {
                        Chip8::print_debug(&format!("0xFX33: Stores the binary-coded decimal representation of VX"));
                        self.memory[self.index as usize] = self.v[((op_code & 0x0F00) >> 8) as usize] / 100;
                        self.memory[(self.index + 1) as usize] = (self.v[((op_code & 0x0F00) >> 8) as usize] / 10) % 10;
                        self.memory[(self.index + 2) as usize] = (self.v[((op_code & 0x0F00) >> 8) as usize] % 100) % 10;
                        self.program_counter += 2;
                    },
                    0x0065 => {
                        Chip8::print_debug(&format!("0xFX65: Fills V0 to VX (including VX) with values from memory starting at address I"));
                        for i in 0x0..(((op_code & 0x0F00) >> 8) + 1) {
                            self.v[i as usize] = self.memory[(self.index + i) as usize];
                        }
                        // Only on original interpreter
                        self.index += ((op_code & 0x0F00) >> 8) + 1;
                        self.program_counter += 2;
                    }
                    _ => {
                        Chip8::print_debug(&format!("Unrecognized op code: {:X?}", op_code));
                        return;
                    }
                }
            },
            _ => {
                Chip8::print_debug(&format!("Unrecognized op code: {:X?}", op_code));
                return;
            }
        }
        self.update_timers();
    }

    fn fetch_opcode(&mut self) {
        let first_byte = self.memory[self.program_counter as usize];
        let second_byte = self.memory[self.program_counter as usize + 1];
        self.op_code = (first_byte as u16) << 8 | second_byte as u16;
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                Chip8::print_debug(&format!("BEEP"));
            }
            self.sound_timer -= 1;
        }
    }

    fn print_debug(msg: &String) {
        let debug = true;
        if debug {
            println!("{}", msg);
        }
    }
}

