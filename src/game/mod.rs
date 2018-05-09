use piston::input::*;
use opengl_graphics::{ GlGraphics };
use graphics::*;

pub mod pieces;

use super::gui::App;
use self::pieces::{Piece, Move};

pub struct Game {
    width: u32, height: u32,
    pieces: Vec<Piece>
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            width, height, 
            pieces: Vec::new()
        }
    }

    pub fn from_vec(width: u32, height: u32, pieces: &Vec<Piece>) -> Game {
        let mut game = Game::new(width, height);

        game.pieces.extend_from_slice(&pieces);

        game
    }
}

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

impl App for Game {
    fn render(&self, args: &RenderArgs, mut gl: GlGraphics) -> GlGraphics {
        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(GREEN, g);

            
        });

        gl
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}
