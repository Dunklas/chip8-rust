mod chip8;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    let rom_bytes = match fs::read(rom_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to read file \"{}\" due to: {}", rom_path, e);
            return;
        }
    };

    let mut chip8 = chip8::new();
    chip8.load_game(rom_bytes.as_slice());

    let debug = false;
    if debug {
        let mut index = 0;
        let rom_bytes_slice = rom_bytes.as_slice();
        while (index < rom_bytes_slice.len() - 1) {
            let first_byte = rom_bytes_slice[index];
            let second_byte = rom_bytes_slice[index + 1];
            let op_code: u16 = (first_byte as u16) << 8 | second_byte as u16;
            println!("{:X?}", op_code);
            index += 2;
        }
    }

    loop {
        chip8.emulate_cycle();
    }
}
