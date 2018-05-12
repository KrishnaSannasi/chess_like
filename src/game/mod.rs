use std::sync::atomic::AtomicPtr;
use std::convert::From;
use std::sync::Arc;

use piston_window::*;
// use piston_window::character::CharacterCache;
// use find_folder::Search;

pub mod pieces;
pub mod action;

use super::gui::{App, Data, AppGraphics, unwrap};
use self::pieces::*;
use self::action::*;
use self::action::composite_vals::*;

const BLACK   : [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE   : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const SELECTED: [f32; 4] = [0.7, 0.7, 0.7, 1.0];

const TON: f32 = 1.0;
const PON: f32 = 0.4;
const OFF: f32 = 0.3;

const SELECTED_MOVE_CAPTURE: [f32; 4] = [TON, PON, OFF, 1.0];
const SELECTED_MOVE_UPGRADE: [f32; 4] = [OFF, TON, PON, 1.0];
const SELECTED_MOVE_MOVE   : [f32; 4] = [PON, OFF, TON, 1.0];

const TEAMS: [[f32; 4]; 3] = 
    [[180.0 / 255.0, 37.5  / 255.0, 180.0 / 255.0, 1.0]
    ,[180.0 / 255.0, 180.0 / 255.0, 37.5  / 255.0, 1.0]
    ,[37.5  / 255.0, 180.0 / 255.0, 180.0 / 255.0, 1.0]];
const TEAMS_DIM: [[f32; 4]; 3] = 
    [[240.0 / 255.0, 50.0   / 255.0, 240.0 / 255.0, 1.0]
    ,[240.0 / 255.0, 240.0  / 255.0, 50.0  / 255.0, 1.0]
    ,[50.0  / 255.0, 240.0  / 255.0, 240.0 / 255.0, 1.0]];

pub struct Game {
    width: u32, height: u32,
    selected_x: u32, selected_y: u32,
    pieces: Vec<Piece>, teams: Vec<Alliance>,
    turn: usize, action_stack: Vec<Action>,
    data: AtomicPtr<Data>,
    window: AtomicPtr<PistonWindow>,
}

// related functions
impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let team_1 = Alliance::new("Team 1", 20, TEAMS[0]);
        let team_2 = Alliance::new("Team 2", 22, TEAMS[1]);
        let team_3 = Alliance::new("Team 2", 24, TEAMS[2]);

        Game::from_vec(width, height, Vec::new(), vec![team_1, team_2, team_3])
    }

    pub fn from_vec(width: u32, height: u32, pieces: Vec<Piece>, teams: Vec<Alliance>) -> Self {
        Self {
            width, height, 
            selected_x: 100, selected_y: 100,
            pieces, teams, turn: 0,
            action_stack: Vec::new(),
            data: AtomicPtr::default(),
            window: AtomicPtr::default(),
        }
    }
}

// immutable functions
impl Game {
    fn get_piece(&self, x: u32, y: u32) -> Option<&Piece> {
        let (x, y) = (x as i32, y as i32);

        for p in &self.pieces {
            if p.x() == x && p.y() == y {
                return Some(p);
            }
        }

        None
    }
    
    fn get_tile_size(&self) -> (f64, f64, f64) {
        let data = unwrap(&self.data);
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
    
    fn to_grid(&self, x: f64, y: f64) -> (u32, u32) {
        let (s, dw, dh) = self.get_tile_size();
        
        let (x, y) = (x - dw, y - dh);

        ((x / s) as u32, (y / s) as u32)
    }

    fn tile_color(&self, i: u32, j: u32, piece: Option<&Piece>) -> [f32; 4] {
        if i == self.selected_x && j == self.selected_y {
            SELECTED
        }
        else if let Some(action) = self.can_move(piece, i, j, false) {
            match action {
                Action::Place(_) => SELECTED,
                Action::Remove(_) => panic!("remove action leaked into draw loop"),
                Action::Move(_, _, _) => SELECTED_MOVE_MOVE,
                Action::Upgrade(_, _) => SELECTED_MOVE_UPGRADE,
                Action::Composite(_) => SELECTED_MOVE_CAPTURE
            }
        }
        else if (i + j) % 2 == 0 {
            TEAMS_DIM[self.turn]
        } else {
            BLACK
        }
    }

    fn print_team_pieces_left(&self) {
        for t in &self.teams {
            println!("team {}: {}", t.name, t.pieces_left);
        }
        println!();
    }

    fn can_move(&self, piece: Option<&Piece>, x: u32, y: u32, ignore_teams: bool) -> Option<Action> {
        let (ix, iy) = (x as i32, y as i32);

        match piece {
            None => None,
            Some(p) => {
                
                if ignore_teams && p.team() != self.turn {
                    None
                } else if !p.can_move(MoveDir::new(ix - p.x(), iy - p.y())) {
                    None
                } else if let Some(other) = self.get_piece(x, y) {
                    if other.team() == p.team() {
                        Some(Action::Upgrade(p.clone(), other.clone()))
                    } else if p.level() + 1 >= other.level() {
                        Some(Action::Composite(Composite::Capture(
                            Arc::new(CaptureVal::from([
                                Action::Remove(other.clone()),
                                Action::Move(p.clone(), ix - p.x(), iy - p.y())
                            ])
                        )) ))
                    } else {
                        None
                    }
                } else {
                    Some(Action::Move(p.clone(), ix - p.x(), iy - p.y()))
                }
            }
        }
    }
}

// mutable functions
impl Game {
    fn inc(&mut self) {
        self.turn = (self.turn + 1) % self.teams.len();
    }

    fn dec(&mut self) {
        let len = self.teams.len();
        self.turn = (self.turn + len - 1) % len;
    }

    fn place(&mut self, x: u32, y: u32, team: usize) -> Result<Action, String> {
        let is_empty = self.get_piece(x, y).is_none();
        
        if self.teams[team].pieces_left == 0 {
            Err(String::from("Cannot place anymore pieces"))
        }
        else if is_empty {
            let piece = Piece::new(x as u16, y as u16, self.turn);
            Ok(Action::Place(piece))
        } else {
            Err(String::from("Cannot place ontop of another piece"))
        }
    }
    
    fn remove_piece_at(&mut self, x: i32, y: i32) -> Result<(), String> {
        let pos = self.pieces.iter().position(|p| p.x() == x && p.y() == y);
        
        match pos {
            Some(index) => {
                self.pieces.remove(index);
                Ok(())
            },
            None =>
                Err(format!("No piece at ({}, {})", x, y))
        }
    }

    fn place_piece(&mut self, p: &Piece) -> Result<(), String> {
        let is_empty = self.get_piece(p.x() as u32, p.y() as u32).is_none();

        if is_empty {
            self.pieces.push(p.clone());
            Ok(())
        } else {
            Err(String::from("Cannot place ontop of another piece"))
        }
    }

    fn remove_piece(&mut self, piece: &Piece) -> Result<(), String> {
        let pos = self.pieces.iter().position(|p| p == piece);

        match pos {
            Some(index) => {
                self.pieces.remove(index);
                Ok(())
            },
            None =>
                Err(format!("Piece does not exists in this game: {}", piece))
        }
    }

    fn do_action(&mut self, action: Action) {
        if let Action::Place(p) = action.clone() {
            self.teams[p.team()].pieces_left -= 1;
            self.print_team_pieces_left();
        }
        self.inc();
        self.action_stack.push(action.clone());
        self.apply_action(action);
    }

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::Place(p) => {
                if let Err(msg) = self.place_piece(&p) {
                    println!("err = {}", msg);
                }
            },
            Action::Remove(p) => {
                if let Err(msg) = self.remove_piece(&p) {
                    println!("err = {}", msg);
                }
            },
            Action::Move(old, dx, dy) => {
                let dir = MoveDir::new(dx, dy);
                let mut new = old.clone();
                new.apply(Move::from(dir));

                self.apply_action(Action::Remove(old));
                self.apply_action(Action::Place(new));
            },
            Action::Upgrade(sacrifice, old) => {
                let mut new = old.clone();
                if let Err(msg) = new.upgrade(&sacrifice) {
                    println!("err = {}", msg);
                    self.dec();
                } else {
                    self.apply_action(Action::Remove(old));
                    self.apply_action(Action::Remove(sacrifice));
                    self.apply_action(Action::Place(new));
                }
            },
            Action::Composite(c) => {
                match c {
                    Composite::Capture(c) => {
                        self.apply_action(c.remove_action.clone());
                        self.apply_action(c.move_action.clone());
                    }
                }
            }
        }
    }

    fn undo_last(&mut self) {
        match self.action_stack.pop() {
            Some(action) => {
                if let Action::Place(p) = action.clone() {
                    self.teams[p.team()].pieces_left += 1;
                            self.print_team_pieces_left();
                }
                self.dec();
                self.undo_action(action)
            },
            None => (),
        }
    }

    fn undo_action(&mut self, action: Action) {
        match action {
            Action::Place(p) => {
                if let Err(msg) = self.remove_piece(&p) {
                    println!("err = {}", msg);
                }
            },
            Action::Remove(p) => {
                if let Err(msg) = self.place_piece(&p) {
                    println!("err = {}", msg);
                }
            },
            Action::Move(old, dx, dy) => {
                let dir = MoveDir::new(dx, dy);
                let mut new = old.clone();
                new.apply(Move::from(dir));

                self.undo_action(Action::Remove(old));
                self.undo_action(Action::Place(new));
            },
            Action::Upgrade(sacrifice, old) => {
                if let Err(msg) = self.remove_piece_at(old.x(), old.y()) {
                    println!("err = {}", msg);
                }

                self.undo_action(Action::Remove(old));
                self.undo_action(Action::Remove(sacrifice));
            },
            Action::Composite(c) => {
                match c {
                    Composite::Capture(c) => {
                        self.undo_action(c.move_action.clone());
                        self.undo_action(c.remove_action.clone());
                    }
                }
            }
        }
    }
}

impl App for Game {
    fn set_data(&mut self, data: AtomicPtr<Data>) {
        self.data = data;
    }

    fn set_window(&mut self, window: AtomicPtr<PistonWindow>) {
        self.window = window;
    }

    fn render(&self, c: Context, g: &mut AppGraphics) {
        use graphics::*;

        let (s, dw, dh) = self.get_tile_size();

        clear(WHITE, g);
        let transform = c.transform.trans(dw, dh).scale(s, s); //.trans((data.screen_width as f64 - sz), (data.screen_height as f64 - sz) / 2.0);
        let piece = self.get_piece(self.selected_x, self.selected_y);

        for i in 0..self.width {
            for j in 0..self.height {
                let c = self.tile_color(i, j, piece);

                let sq = rectangle::square(i as f64, j as f64, 1.0);
                rectangle(c, sq, transform, g);
            }
        }

        for p in &self.pieces {
            let sq = rectangle::square(p.x() as f64 + 0.3, p.y() as f64 + 0.3, 0.4);
            ellipse(self.teams[p.team()].color, sq, transform, g);
            if p.level() > 1 {
                let sq = rectangle::square(p.x() as f64 + 0.1, p.y() as f64 + 0.1, 0.2);
                ellipse(self.teams[p.team()].color, sq, transform, g);
            }
            if p.level() > 2 {
                let sq = rectangle::square(p.x() as f64 + 0.7, p.y() as f64 + 0.1, 0.2);
                ellipse(self.teams[p.team()].color, sq, transform, g);
            }
            if p.level() > 3 {
                let sq = rectangle::square(p.x() as f64 + 0.7, p.y() as f64 + 0.7, 0.2);
                ellipse(self.teams[p.team()].color, sq, transform, g);
            }
            if p.level() > 4 {
                let sq = rectangle::square(p.x() as f64 + 0.1, p.y() as f64 + 0.7, 0.2);
                ellipse(self.teams[p.team()].color, sq, transform, g);
            }
            if p.level() > 5 {
                let sq = rectangle::square(p.x() as f64 + 0.4, p.y() as f64 + 0.4, 0.2);
                ellipse(self.tile_color(p.x() as u32, p.y() as u32, None), sq, transform, g);
            }
        }
        
        /*
        let font = Search::ParentsThenKids(3, 3)
                        .for_folder("res").unwrap()
                        //.join("FiraSans-Regular.ttf")
                        //.join("Verdana.ttf")
                        .join("arial.ttf")
                        ;
        let settings = TextureSettings::new()
                                    .mipmap(Filter::Nearest);
        // text handling
        let ref mut cache = Glyphs::new(font, unwrap(&self.window).factory.clone(), settings).unwrap();
        let (size, text) = (16, "Hello World");
        let transform = c.transform.trans(510.0 + cache.width(size, text).unwrap(), 200.0);
        let _ = graphics::text(BLACK, size, text, cache, transform, g);
        */
    }
    
    fn update(&mut self, _args: &UpdateArgs) {
        // println!("ups = {}", 1.0 / args.dt);
        // println!("{:?}", self.action_stack);
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Z => {
                self.undo_last();
            }
            _ => (),
        }
    }
    
    fn handle_mouse(&mut self, mouse_button: MouseButton, mouse_x: f64, mouse_y: f64) {
        let (x, y) = self.to_grid(mouse_x, mouse_y);
        let (sx, sy) = (self.selected_x, self.selected_y);
        let mut deselect = false;

        match mouse_button {
            MouseButton::Left => {
                if sx == x && sy == y {
                    let turn = self.turn;
                    let place_action = self.place(x, y, turn);
                    match place_action {
                        Ok(action) => {
                            self.do_action(action);
                        },
                        Err(msg) => {
                            println!("msg = {}", msg)
                        }
                    }
                } else {
                    if let Some(action) = self.can_move(self.get_piece(sx, sy), x, y, true) {
                        self.do_action(action);
                        deselect = true;
                    }
                }

                if deselect {
                    self.selected_x = 100;
                    self.selected_y = 100;
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
    }
}