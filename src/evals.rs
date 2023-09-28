use crate::components::{board::Board, constants::ScoreType};

pub mod cell_score;

pub trait EvalTrait {
    fn eval(board: Board) -> ScoreType;
}