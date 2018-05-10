use piston::input::*;
use opengl_graphics::{ GlGraphics };

pub mod pieces;
pub mod action;

use super::gui::{App, Data};
use self::action::Action;
use self::pieces::{Piece, Alliance, MoveDir, Move};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SELECTED: [f32; 4] = [0.7, 0.7, 0.1, 1.0];
const SELECTED_MOVE: [f32; 4] = [0.4, 0.4, 0.0, 1.0];
const TEAMS: [[f32; 4]; 2] = 
    [[183.0 / 255.0, 76.0  / 255.0, 55.0  / 255.0, 1.0]
    ,[72.0  / 255.0, 179.0 / 255.0, 200.0 / 255.0, 1.0]];
const TEAMS_DIM: [[f32; 4]; 2] = 
    [[183.0 / 255.0, 76.0  / 255.0, 55.0  / 255.0, 0.5]
    ,[72.0  / 255.0, 179.0 / 255.0, 200.0 / 255.0, 0.5]];

pub struct Game {
    width: u32, height: u32,
    selected_x: u32, selected_y: u32,
    pieces: Vec<Piece>, teams: Vec<Alliance>,
    turn: usize, action_stack: Vec<Action>
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let team_one: Alliance = Alliance::new("Team 1", TEAMS[0]);
        let team_two: Alliance = Alliance::new("Team 2", TEAMS[1]);

        Game::from_vec(width, height, Vec::new(), vec![team_one, team_two])
    }

    pub fn from_vec(width: u32, height: u32, pieces: Vec<Piece>, teams: Vec<Alliance>) -> Game {
        Game {
            width, height, 
            selected_x: 100, selected_y: 100,
            pieces, teams, turn: 0,
            action_stack: Vec::new()
        }
    }

    fn inc(&mut self) {
        self.turn = (self.turn + 1) % self.teams.len();
    }

    fn cur_team(&self) -> Alliance {
        self.teams[self.turn].clone()
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

    fn get_piece_mut(&mut self, x: u32, y: u32) -> Option<&mut Piece> {
        let (x, y) = (x as i32, y as i32);

        for p in &mut self.pieces {
            if p.x() == x && p.y() == y {
                return Some(p);
            }
        }

        None
    }

    fn remove_piece(&mut self, x: u32, y: u32) -> Result<(), String> {
        let pos = self.pieces.iter().position(|p| p.x() == x as i32 && p.y() == y as i32);
        match pos {
            Some(index) => {
                self.pieces.remove(index);
                Ok(())
            },
            None =>
                Err(String::from("No piece at position"))
        }
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

    fn can_move(&self, piece: Option<&Piece>, x: u32, y: u32) -> bool {
        let (ix, iy) = (x as i32, y as i32);

        match piece {
            None => false,
            Some(p) => {
                if !p.can_move(MoveDir::new(ix - p.x(), iy - p.y())) {
                    false
                } else if let Some(other) = self.get_piece(x, y) {
                    other.team() != p.team()
                } else {
                    true
                }
            }
        }
    }
}

impl App for Game {
    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics, data: &Data) {
        use graphics::*;

        let (s, dw, dh) = self.get_tile_size(data);

        gl.draw(args.viewport(), |c, g| {
            // Clear the screen.
            clear(WHITE, g);
            let transform = c.transform.trans(dw, dh).scale(s, s); //.trans((data.screen_width as f64 - sz), (data.screen_height as f64 - sz) / 2.0);
            let piece = self.get_piece(self.selected_x, self.selected_y);
            
            for i in 0..self.width {
                for j in 0..self.height {
                    let c = {
                        if i == self.selected_x && j == self.selected_y {
                            SELECTED
                        }
                        else if self.can_move(piece, i, j) {
                            SELECTED_MOVE
                        }
                        else if (i + j) % 2 == 0 {
                            TEAMS_DIM[self.turn]
                        } else {
                            BLACK
                        }
                    };

                    let sq = rectangle::square(i as f64, j as f64, 1.0);
                    rectangle(c, sq, transform, g);
                }
            }

            for p in &self.pieces {
                let sq = rectangle::square(p.x() as f64 + 0.3, p.y() as f64 + 0.3, 0.4);
                ellipse(p.team().color, sq, transform, g);
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs, _data: &Data) {
        // println!("ups = {}", 1.0 / args.dt);
    }

    fn handle_mouse(&mut self, mouse_button: MouseButton, data: &Data) {
        let (x, y) = self.to_grid(data.mouse_x, data.mouse_y, data);
        let (sx, sy) = (self.selected_x, self.selected_y);
        let (mut do_remove, mut deselect) = (None, false);
        let mut action = None;

        match mouse_button {
            MouseButton::Left => {
                if sx == x && sy == y {
                    let turn = self.turn;
                    match self.place(x, y, turn) {
                        Ok(()) =>  {
                            deselect = true;
                            action = Some(Action::PLACE(x, y))
                        }
                        Err(msg) => {
                            println!("{}", msg);
                            println!("{:?}", self.get_piece(x, y));
                        }
                    };
                } else {
                    let dir = MoveDir::new(x as i32 - sx as i32, y as i32 - sy as i32);
                    let cur_team = self.cur_team();

                    if self.can_move(self.get_piece(sx, sy), x, y) {
                        match self.get_piece(sx, sy) {
                            None => (),
                            Some(piece1) => {
                                if *piece1.team() == cur_team {
                                    match self.get_piece(x, y) {
                                        None => (),
                                        Some(piece2) => {
                                            do_remove = Some(piece1.team() != piece2.team());
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(b) = do_remove {
                            if b {
                                if let Err(msg) = self.remove_piece(x, y) {
                                    println!("error = {}", msg);
                                }
                            }
                        }

                        match self.get_piece_mut(sx, sy) {
                            Some(p) => {
                                if (*p.team() == cur_team) && (do_remove.is_none() || do_remove.unwrap()) {
                                    p.apply(Move::from(dir));
                                    action = Some(Action::MOVE(sx, sy, dir.dx(), dir.dy()));
                                    deselect = true;
                                }
                            },
                            None => ()
                        }
                    }
                }

                if deselect {
                    self.selected_x = 100;
                    self.selected_y = 100;
                    self.inc();
                } else {
                    self.selected_x = x;
                    self.selected_y = y;
                }
            },
            _ => {
                self.selected_x = 100;
                self.selected_y = 100;
            }
        }; // end match
        match action {
            Some(a) => self.action_stack.push(a),
            None => (),
        };
    }
}


