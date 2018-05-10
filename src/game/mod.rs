use piston::input::*;
use opengl_graphics::{ GlGraphics };
use std::cmp::{min, max};

pub mod pieces;

use super::gui::{App, Data};
use self::pieces::{Piece, Move};

const padding: f64 = 2.0;

pub struct Game {
    width: u32, height: u32,
    pieces: Vec<Piece>,
    selected_x: u32, selected_y: u32
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            width, height, 
            selected_x: 100, selected_y: 100,
            pieces: Vec::new()
        }
    }

    pub fn from_vec(width: u32, height: u32, pieces: &Vec<Piece>) -> Game {
        let mut game = Game::new(width, height);

        game.pieces.extend_from_slice(&pieces);

        game
    }

    fn get_tile_size(&self, data: &Data) -> (f64, f64, f64) {
        let (s1, s2) = (data.screen_width as f64 / self.width as f64, data.screen_height as f64 / self.height as f64);

        let s = {
            if s1 < s2 {
                s1
            } else {
                s2
            }
        };

        (s, (data.screen_width  - s as u32 * self.width ) as f64 / 2.0, 
            (data.screen_height - s as u32 * self.height) as f64 / 2.0)
    }

    fn to_grid(&self, x: f64, y: f64, data: &Data) -> (u32, u32) {
        let (s, dw, dh) = self.get_tile_size(data);
        
        let (x, y) = (x - dw, y - dh);

        ((x / s) as u32, (y / s) as u32)
    }
}

const BACKGROUND: [f32; 4] = [0.2, 0.35, 0.5, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SELECTED: [f32; 4] = [0.3, 0.7, 0.5, 1.0];

impl App for Game {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, data: &Data) {
        use graphics::*;

        let (s, dw, dh) = self.get_tile_size(data);

        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(BACKGROUND, g);
            let transform = c.transform.trans(dw, dh).scale(s, s); //.trans((data.screen_width as f64 - sz), (data.screen_height as f64 - sz) / 2.0);
            for i in 0..self.width {
                for j in 0..self.height {
                    let c = {
                        if i == self.selected_x && j == self.selected_y {
                            SELECTED
                        }
                        else if (i + j) % 2 == 0 {
                            WHITE
                        } else {
                            BLACK
                        }
                    };

                    let sq = rectangle::square(i as f64, j as f64, 1.0);
                    rectangle(c, sq, transform, g);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, data: &Data) {
        
    }

    fn handle_mouse(&mut self, mouse_button: MouseButton, data: &Data) {
        let (x, y) = self.to_grid(data.mouse_x, data.mouse_y, data);

        match mouse_button {
            MouseButton::Left => {
                self.selected_x = x;
                self.selected_y = y;
            },
            _ => {
                self.selected_x = 100;
                self.selected_y = 100;
            }
        }
    }
}


