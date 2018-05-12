use std::fmt::{Debug, Formatter, Result};
use std::sync::Arc;

use game::pieces::Piece;

pub mod composite_vals;

use self::composite_vals::*;

#[derive(Clone)]
pub enum Action {
    Place(Piece),
    Remove(Piece),
    Move(Piece, i32, i32),
    Upgrade(Piece, Piece),
    Composite(Composite),
}

impl Debug for Action {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Action::Place(_) => write!(f, "Place"),
            &Action::Remove(_) => write!(f, "Remove"),
            &Action::Move(_,_,_) => write!(f, "Move"),
            &Action::Upgrade(_,_) => write!(f, "Upgrade"),
            &Action::Composite(ref c) => write!(f, "Composite {:?}", c),
        }
    }
}

#[derive(Clone)]
pub enum Composite {
    Capture(Arc<CaptureVal>)
}

impl Debug for Composite {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            &Composite::Capture(_) => write!(f, "Capture"),
        }
    }
}
