mod chip8;
use std::env;
use std::fs;

extern crate rand;
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
        use graphics::*;
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const PIXEL_SIZE: f64 = 16.0;

        if self.chip8.draw {
            self.chip8.draw = false;
            let gfx = self.chip8.gfx;
            self.gl.draw(args.viewport(), |c, gl| {
              clear(BLACK, gl);
              for y in 0..32 {
                  for x in 0..64 {
                      if gfx[(y * 64) + x] == 1 {
                          let rect = rectangle::square(0.0, 0.0, PIXEL_SIZE);
                          let transform = c
                            .transform
                            .trans(x as f64 * PIXEL_SIZE, y as f64 * PIXEL_SIZE);
                          rectangle(WHITE, rect, transform, gl);
                      }
                  }
              }
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
    let mut window: Window = WindowSettings::new("chip8", [1024, 512])
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
