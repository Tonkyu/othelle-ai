use crate::components::{board::Board, constants::{ScoreType, TOP_BIT, BOARD_SIZE}, bitboard::BitBoard};

use super::EvalTrait;

pub struct CellEval {}

impl EvalTrait for CellEval {
    fn eval(board: Board) -> ScoreType {
        let mut res_score: ScoreType = 0;
        const SCORE_TABLE: [ScoreType; BOARD_SIZE] = [
            30, -12, 0, -1, -1, 0, -12, 30,
            -12, -15, -3, -3, -3, -3, -15, -12,
            0, -3, 0, -1, -1, 0, -3, 0,
            -1, -3, -1, -1, -1, -1, -3, -1,
            -1, -3, -1, -1, -1, -1, -3, -1,
            0, -3, 0, -1, -1, 0, -3, 0,
            -12, -15, -3, -3, -3, -3, -15, -12,
            30, -12, 0, -1, -1, 0, -12, 30
        ];

        let mut mask: BitBoard = TOP_BIT;
        for i in 0..BOARD_SIZE {
            if board.state.player_bit & mask != 0 {
                res_score += SCORE_TABLE[i];
            }
            if board.state.opponent_bit & mask != 0 {
                res_score -= SCORE_TABLE[i];
            }
            mask >>= 1;
        }
        res_score
    }
}