pub fn new() -> Chip8 {
    Chip8 {
        op_code: 0,
        memory: [0; 4096],
        v: [0; 16],
        index: 0,
        program_counter: 0,
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
}