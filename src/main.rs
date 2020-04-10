mod chip8;
use std::env;
use std::fs;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

struct Game {
    gl: GlGraphics,
    chip8: chip8::Chip8,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        if self.chip8.draw {
            self.gl.draw(args.viewport(), |c, gl| {
              graphics::clear(GREEN, gl)
            });
        }
    }
    fn update(&mut self, args: &UpdateArgs) {
        self.chip8.emulate_cycle();
    }
}

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

    let chip8 = chip8::new(rom_bytes.as_slice());

    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("chip8", [256, 128])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game{
        gl: GlGraphics::new(opengl),
        chip8: chip8,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}
