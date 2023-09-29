use std::time::Instant;

use super::Agent;
use crate::{components::{action::Action, constants::{ScoreType, INF, TIME_LIMT}, board::Board, enums::BoardStatus}, evals::{cell_score::CellEval, EvalTrait}};

pub struct MiniMaxAgent {
    pub depth: i32,
    pub rest_time:  u128,    // micro sec
}

impl Agent for MiniMaxAgent {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        let now = Instant::now();
        let mut best_action: Option<Action> = None;
        let mut best_score: ScoreType = -INF;
        for action in board.legal_actions() {
            let next_board: Board = board.play_onestep(action);
            let score: ScoreType = -self.minimax_score(&next_board, self.depth, now);
            if score > best_score {
                best_score = score;
                best_action = Some(action);
            }
        }
        best_action
    }
}

impl MiniMaxAgent {
    fn minimax_score(&self, board: &Board, depth: i32, now: Instant) -> ScoreType {
        if TIME_LIMT < now.elapsed().as_micros() + self.rest_time {
            return CellEval::eval(*board);
        }

        if board.status() == BoardStatus::Finished || depth == 0 {
            return CellEval::eval(*board);
        }
        let legal_actions: Vec<Action> = board.legal_actions();
        if legal_actions.is_empty() {
            match board.status() {
                BoardStatus::Finished => { return CellEval::eval(*board); },
                BoardStatus::Pass => {
                    let next_board: Board = (*board).play_pass();
                    return -self.minimax_score(&next_board, depth, now);
                },
                _ => {},
            }
        }
        let mut best_score: ScoreType = -INF;
        for action in legal_actions {
            let next_board: Board = (*board).play_onestep(action);
            let score: ScoreType = -self.minimax_score(&next_board, depth-1, now);
            if score > best_score {
                best_score = score;
            }
        }
        best_score
    }
}
