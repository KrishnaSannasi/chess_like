use std::ops::Shl;

#[derive(Debug, Clone, PartialEq)]
pub struct Alliance {
    pub color: [f32; 4],
    pub name: String
}

impl Alliance {
    pub fn new(name: &str, color: [f32; 4]) -> Self {
        Self { name: String::from(name), color }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    x: i32, y: i32, moved: bool,
    team: Alliance, poss_moves: MoveDirSet
}

impl Piece {
    pub fn new(x: u16, y: u16, team: &Alliance) -> Self {
        Self {
            x: x as i32, y: y as i32,
            moved: false, team: team.clone(),
            poss_moves: MoveDirSet::from(vec![(1,0), (-1,0), (0,1), (0,-1)])
        }
    }

    pub fn apply(&mut self, m: Move) {
        self.x += m.dx;
        self.y += m.dy;
        self.moved = true;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
    
    pub fn team(&self) -> &Alliance {
        &self.team
    }

    pub fn can_move(&self, dir: MoveDir) -> bool {
        self.poss_moves.clone().into_iter().position(|pm| pm == dir).is_some()
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
}

#[derive(Debug, Clone)]
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
        let mut moves = Vec::new();

        for (dx, dy) in del {
            moves.push(MoveDir::new(dx, dy));
        }
        
        Self {
            moves
        }
    }
}

impl IntoIterator for MoveDirSet {
    type Item = MoveDir;
    type IntoIter = MoveDirSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        MoveDirSetIterator::new(self)
    }

}

impl Shl<MoveDir> for MoveDirSet {
    type Output = MoveDirSet;

    fn shl(mut self, rhs: MoveDir) -> Self::Output {
        self.moves.push(rhs);

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