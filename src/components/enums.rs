use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Turn {
    Black,
    White,
    Draw,
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

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            Turn::Black => "Black",
            Turn::White => "White",
            Turn::Draw => "Draw",
        };

        write!(f, "{}", value)
    }
}



#[derive(PartialEq)]
pub enum BoardStatus {
    Usual,
    Pass,
    Finished,
}

#[derive(PartialEq)]
pub enum WinningStatus {
    Win,
    Lose,
    Draw,
    NotFinished,
}