#[derive(Debug, Clone)]
pub struct Piece {
    x: i32, y: i32, moved: bool
}

pub struct Move {
    dx: i32, dy: i32
}

impl Piece {
    pub fn new(x: u16, y: u16) -> Piece {
        Piece {
            x: x as i32, y: y as i32,
            moved: false
        }
    }

    pub fn apply(&mut self, m: Move) {
        self.x += m.dx;
        self.y += m.dy;
        self.moved = true;
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