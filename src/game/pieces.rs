#[derive(Debug, Clone, PartialEq)]
pub struct Alliance {
    pub color: [f32; 4],
    pub name: String
}

impl Alliance {
    pub fn new(name: &str, color: [f32; 4]) -> Alliance {
        Alliance { name: String::from(name), color }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    x: i32, y: i32, moved: bool,
    team: Alliance
}

pub struct Move {
    dx: i32, dy: i32
}

impl Piece {
    pub fn new(x: u16, y: u16, team: &Alliance) -> Piece {
        Piece {
            x: x as i32, y: y as i32,
            moved: false, team: team.clone()
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
}

impl Move {
    pub fn new(dx: i32, dy: i32) -> Move {
        Move {dx, dy}
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