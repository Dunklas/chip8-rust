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
    chip8.load_game(rom_bytes.as_slice())
}
