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
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, Key};
use piston::window::WindowSettings;
use piston::input::*;

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
        if self.chip8.wait_keys {
            return;
        }
        self.chip8.emulate_cycle();
    }

    fn key_pressed(&mut self, key: Key) {
        self.chip8.wait_keys = false;
        Game::update_keys(key, 1, &mut self.chip8.keys);
    }

    fn key_released(&mut self, key: Key) {
        self.chip8.wait_keys = false;
        Game::update_keys(key, 0, &mut self.chip8.keys);
    }

    fn update_keys(key: Key, state: u8, keys: &mut [u8; 16]) {
        match key {
            Key::D1 => {
                keys[0x1] = state;
            },
            Key::D2 => {
                keys[0x2] = state;
            },
            Key::D3 => {
                keys[0x3] = state;
            },
            Key::D4 => {
                keys[0xC] = state;
            },
            Key::Q => {
                keys[0x4] = state;
            },
            Key::W => {
                keys[0x5] = state;
            },
            Key::E => {
                keys[0x6] = state;
            },
            Key::R => {
                keys[0xD] = state;
            }
            Key::A => {
                keys[0x7] = state;
            },
            Key::S => {
                keys[0x8] = state;
            },
            Key::D => {
                keys[0x9] = state;
            },
            Key::F => {
                keys[0xE] = state;
            },
            Key::Z => {
                keys[0xA] = state;
            },
            Key::X => {
                keys[0x0] = state;
            },
            Key::C => {
                keys[0xB] = state;
            },
            Key::V => {
                keys[0xF] = state;
            },
            _ => {
                // Do nothing
            }
        }
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

    let event_settings = EventSettings{
        max_fps: 500,
        ups: 500,
        ups_reset: 5,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };
    let mut events = Events::new(event_settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }

        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(key) => {
                    game.key_released(key);
                },
                _ => {
                    // Do nothing
                }
            }
        }
    }
}
