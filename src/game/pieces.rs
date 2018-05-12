use std::ops::{Shl, Index, Add};
use std::fmt::{Display, Formatter, Result};
use std::result;

use rand::{Rng, thread_rng};

#[derive(Debug, Clone, PartialEq)]
pub struct Alliance {
    pub color: [f32; 4],
    pub name: String,
    pub pieces_left: u32
}

impl Alliance {
    pub fn new(name: &str, pieces_left: u32, color: [f32; 4]) -> Self {
        Self { name: String::from(name), color, pieces_left }
    }
}

impl Display for Alliance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    x: i32, y: i32, team: usize,
    pub poss_moves: MoveDirSet, level: u32
}

impl Piece {
    pub fn new(x: u16, y: u16, team: usize) -> Self {
        Self {
            x: x as i32, y: y as i32, team, level: 1,
            poss_moves: MoveDirSet::from(vec![(1,0), (-1,0), (0,1), (0,-1)])
        }
    }

    pub fn apply(&mut self, m: Move) {
        self.x += m.dx;
        self.y += m.dy;
    }

    pub fn upgrade(&mut self, other: &Piece) -> result::Result<(), String> {
        if self.level + other.level > 6 {
            return Err(format!("that level is too high to control"));
        }

        let mut poss_moves = self.poss_moves.clone();
        let mut fail = false;

        for om in other.poss_moves.clone() {
            let mut vec = vec![];

            for pm in self.poss_moves.clone() {
                let tm = om.clone() + pm.clone();
                if (tm.dx() != 0 || tm.dy() != 0) && poss_moves.clone().into_iter().position(|m| m == tm).is_none() {
                    vec.push(tm);
                }
            }
            if vec.len() == 0 {
                fail = true;
            } else {
                let index: f64 = thread_rng().gen();
                let index = (vec.len() as f64 * index) as usize;
                poss_moves.moves.push(vec[index]);
            }
        }
        if fail {
            self.upgrade(other)?;
        } else {
            self.level += other.level;
            self.poss_moves = poss_moves;
        }
        Ok(())
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
    
    pub fn team(&self) -> usize {
        self.team
    }
    
    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn can_move(&self, dir: MoveDir) -> bool {
        self.poss_moves.moves.contains(&dir) //.clone().into_iter().position(|pm| pm == dir).is_some()
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "piece @({}, {}, {}) -> {}", self.x, self.y, self.team, self.poss_moves)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveDir {
    dx: i32, dy: i32
}

impl MoveDir {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self {
            dx, dy
        }
    }

    pub fn dx(&self) -> i32 {
        self.dx
    }

    pub fn dy(&self) -> i32 {
        self.dy
    }
}

impl Add for MoveDir {
    type Output = MoveDir;

    fn add(self, other: MoveDir) -> Self::Output {
        MoveDir::new(self.dx + other.dx, self.dy + other.dy)
    }
}

impl Display for MoveDir {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "$({}, {})", self.dx, self.dy)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveDirSet {
    moves: Vec<MoveDir>
}

impl MoveDirSet {
    pub fn new() -> Self {
        Self {
            moves: vec![]
        }
    }

    pub fn from(del: Vec<(i32, i32)>) -> Self {
        let mut set = Self::new();

        for (dx, dy) in del {
            set = set << MoveDir::new(dx, dy);
        }
        
        set
    }
}

impl Display for MoveDirSet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut comma_separated = String::new();

        for mv in &self.moves {
            comma_separated.push_str(", ");
            comma_separated.push_str(format!("$({}, {})", mv.dx, mv.dy).as_str());
        }

        write!(f, "{{ {} }}", &comma_separated[2..])
    }
}

impl IntoIterator for MoveDirSet {
    type Item = MoveDir;
    type IntoIter = MoveDirSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        MoveDirSetIterator::new(self)
    }
}

impl Index<usize> for MoveDirSet {
    type Output = MoveDir;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.moves[idx]
    }
}

impl Shl<MoveDir> for MoveDirSet {
    type Output = MoveDirSet;

    fn shl(mut self, rhs: MoveDir) -> Self::Output {
        if !self.moves.contains(&rhs) && (rhs.dx != 0 || rhs.dy != 0) {
            self.moves.push(rhs);
        }

        self
    }
}

pub struct MoveDirSetIterator {
    set: MoveDirSet,
    cur: usize
}

impl MoveDirSetIterator {
    fn new(set: MoveDirSet) -> Self {
        Self {
            set, cur: 0
        }
    }
}

impl Iterator for MoveDirSetIterator {
    type Item = MoveDir;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.set.moves.len() {
            let val = self.set.moves[self.cur];
        
            self.cur += 1;

            Some(val)
        } else {
            None
        }
    }
}

pub struct Move {
    dx: i32, dy: i32
}

impl Move {
    pub fn from(move_dir: MoveDir) -> Self {
        Self {
            dx: move_dir.dx, dy: move_dir.dy
        }
    }
}

#[cfg(test)]
pub mod test {

    #[test]
    fn new() {
        let p = super::Piece::new(3, 1);

        assert_eq!(p.x, 3);
        assert_eq!(p.y, 1);
    }
}