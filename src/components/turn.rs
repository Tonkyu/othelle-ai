
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Turn {
    Black,
    White
}

pub trait TurnTrait {
    fn reverse(&self) -> Turn;
}
pub const FIRST_TURN: Turn = Turn::Black;

impl TurnTrait for Turn {
    fn reverse(&self) -> Turn {
        if *self == Turn::Black {
            Turn::White
        } else {
            Turn::Black
        }
    }
}