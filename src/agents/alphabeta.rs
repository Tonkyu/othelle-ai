use std::time::Instant;

use super::Agent;
use crate::{components::{action::Action, constants::{ScoreType, BoardStatus, INF, TIME_LIMT}, board::Board}, evals::{cell_score::CellEval, EvalTrait}};

pub struct AlphaBetaAgent {
    pub depth: i32,
    pub rest_time:  u128,    // micro sec
}

impl Agent for AlphaBetaAgent {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        let now = Instant::now();
        let mut best_action: Option<Action> = None;
        let mut alpha: ScoreType = -INF;
        let beta: ScoreType = INF;
        for action in board.legal_actions() {
            let next_board: Board = board.play_onestep(action);
            let score: ScoreType = -self.alpha_beta_score(&next_board, self.depth, -beta, -alpha, now);
            if score > alpha {
                alpha = score;
                best_action = Some(action);
            }
        }
        best_action
    }
}

impl AlphaBetaAgent {
    fn alpha_beta_score(&self, board: &Board, depth: i32, mut alpha: ScoreType, beta: ScoreType, now: Instant) -> ScoreType {
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
                    return -self.alpha_beta_score(&next_board, depth, alpha, beta, now);
                },
                _ => {},
            }
        }
        for action in legal_actions {
            let next_board: Board = (*board).play_onestep(action);
            let score: ScoreType = -self.alpha_beta_score(&next_board, depth-1, -beta, -alpha, now);
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                return alpha
            }
        }
        alpha
    }
}
