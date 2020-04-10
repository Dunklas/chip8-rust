pub fn new(rom_bytes: &[u8]) -> Chip8 {
    let mut chip8 = Chip8 {
        op_code: 0,
        memory: [0; 4096],
        v: [0; 16],
        index: 0,
        program_counter: 0x200,
        gfx: [0; 62 * 32],
        delay_timer: 0,
        sound_timer: 0,
        stack: [0; 16],
        stack_pointer: 0,
        key: [0; 16],
        draw: false
    };

    let font_set = font_set();
    for (i, _byte) in font_set.iter().enumerate() {
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
    pub gfx: [u8; 62 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: u16,
    key: [u8; 16],
    pub draw: bool
}

impl Chip8 {

    pub fn emulate_cycle(&mut self) {
        self.draw = false;
        let op_code = self.fetch_opcode();
        Chip8::print_debug(&format!("OP: {:X?}", op_code));
        match op_code & 0xF000 {
            0x0000 => {
                match op_code & 0x000F {
                    0x0000 => {
                        Chip8::print_debug(&format!("0x00E0: Clear screen"));
                        // TODO: IMPLEMENT!
                        self.program_counter += 2;
                    },
                    0x000E => {
                        Chip8::print_debug(&format!("0x00EE: Return from subroutine"));
                        self.stack_pointer -= 1;
                        self.program_counter = self.stack[self.stack_pointer as usize];
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
                self.v[((op_code & 0x0F00) >> 8) as usize] = (op_code & 0x00FF) as u8;
                self.program_counter += 2;
            }
            0xA000 => {
                Chip8::print_debug(&format!("0xANNN: Sets I to the address NNN"));
                self.index = op_code & 0x0FFF;
                self.program_counter += 2;
            },
            0xD000 => {
                Chip8::print_debug(&format!("0xDXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels"));
                let x = self.v[((op_code & 0x0F00) >> 8) as usize] as u16;
                let y = self.v[((op_code & 0x00F0) >> 4) as usize] as u16;
                let height = op_code & 0x000F;

                self.v[0xF] = 0;
                for y_line in 0..height {
                    let pixel = self.memory[(self.index + y_line) as usize] as u8;
                    for x_line in 0..8 {
                        if pixel & (0x80 >> x_line) != 0 {
                            if self.gfx[(x + x_line + ((y + y_line) * 64)) as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[(x + x_line + ((y + y_line) * 64)) as usize] ^= 1;
                        }
                    }
                }

                self.draw = true;
                self.program_counter += 2;
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
                        let mut offset = self.index;
                        for i in 0x0..0xF {
                            self.v[i as usize] = self.memory[offset as usize];
                            offset += 1;
                        }
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

    fn fetch_opcode(&mut self) -> u16 {
        let first_byte = self.memory[self.program_counter as usize];
        let second_byte = self.memory[self.program_counter as usize + 1];
        let op_code: u16 = (first_byte as u16) << 8 | second_byte as u16;
        return op_code;
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

