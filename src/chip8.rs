pub fn new() -> Chip8 {
    Chip8 {
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
        key: [0; 16]
    }
}

pub struct Chip8 {
    op_code: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    index: u16,
    program_counter: u16,
    gfx: [u8; 62 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    stack_pointer: u16,
    key: [u8; 16]
}

impl Chip8 {
    pub fn load_game(&mut self, rom_bytes: &[u8]) {
        for (i, &byte) in rom_bytes.iter().enumerate() {
            self.memory[i + 0x200] = byte;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let op_code = self.fetch_opcode();
        match op_code & 0xF000 {
            0x1000 => {
                println!("Jump!")
            },
            _ => {
                println!("Unrecognized op code");
                return;
            }
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let first_byte = self.memory[self.program_counter as usize];
        let second_byte = self.memory[self.program_counter as usize + 1];
        let op_code: u16 = (first_byte as u16) << 8 | second_byte as u16;
        // println!("First byte: {:#018b}. Second byte: {:#018b}. Op code: {:#018b}", first_byte, second_byte, op_code);
        return op_code;
    }
}