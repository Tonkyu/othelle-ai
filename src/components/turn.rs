pub type Turn = bool;

pub const BLACK_TURN: Turn = true;
pub const WHITE_TURN: Turn = false;
pub const FIRST_TURN: Turn = BLACK_TURN;

pub trait TurnTrait {
    fn swap(&mut self);
}

impl TurnTrait for Turn {
    fn swap(&mut self) {
        if *self == BLACK_TURN {
            *self = WHITE_TURN;
        } else {
            *self = BLACK_TURN;
        }
    }
}