use piston::input::*;
use opengl_graphics::{ GlGraphics };
use std::cmp::{min, max};

pub mod pieces;

use super::gui::{App, Data};
use self::pieces::{Piece, Alliance, Move};

const BACKGROUND: [f32; 4] = [0.2, 0.35, 0.5, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SELECTED: [f32; 4] = [0.7, 0.5, 0.3, 1.0];

pub struct Game {
    width: u32, height: u32,
    selected_x: u32, selected_y: u32,
    pieces: Vec<Piece>, teams: Vec<Alliance>,
    turn: usize
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let team_one: Alliance = Alliance::new("Team 1", [138.0 / 255.0, 219.0 / 255.0, 114.0 / 255.0, 1.0]);
        let team_two: Alliance = Alliance::new("Team 2", [117.0 / 255.0, 36.0  / 255.0, 141.0 / 255.0, 1.0]);

        Game::from_vec(width, height, Vec::new(), vec![team_one, team_two])
    }

    pub fn from_vec(width: u32, height: u32, pieces: Vec<Piece>, teams: Vec<Alliance>) -> Game {
        Game {
            width, height, 
            selected_x: 100, selected_y: 100,
            pieces, teams, turn: 0
        }
    }

    fn inc(&mut self) {
        self.turn = (self.turn + 1) % self.teams.len();
    }

    fn cur_team(&self) -> &Alliance {
        &self.teams[self.turn]
    }

    fn place(&mut self, x: u32, y: u32, team: usize) -> Result<(), String> {
        let p = self.get_piece(x, y).is_none();

        if p {
            self.pieces.push(Piece::new(x as u16, y as u16, &self.teams[team]));
            Ok(())
        } else {
            Err(String::from("Cannot place ontop of another piece"))
        }
    }

    fn get_piece(&self, x: u32, y: u32) -> Option<&Piece> {
        let (x, y) = (x as i32, y as i32);

        for p in &self.pieces {
            if p.x() == x && p.y() == y {
                return Some(p);
            }
        }

        None
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

            for p in &self.pieces {
                let sq = rectangle::square(p.x() as f64 + 0.05, p.y() as f64 + 0.05, 0.9);
                ellipse(p.team().color, sq, transform, g);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs, data: &Data) {
        // println!("ups = {}", 1.0 / args.dt);
    }

    fn handle_mouse(&mut self, mouse_button: MouseButton, data: &Data) {
        let (x, y) = self.to_grid(data.mouse_x, data.mouse_y, data);

        match mouse_button {
            MouseButton::Left => {
                if self.selected_x == x && self.selected_y == y {
                    let turn = self.turn;
                    match self.place(x, y, turn) {
                        Ok(()) => (),
                        Err(msg) => println!("{}", msg)
                    };
                    self.inc();
                }

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


