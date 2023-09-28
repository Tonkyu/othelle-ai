use super::Agent;
use crate::{components::{action::Action, constants::{ScoreType, BoardStatus, INF}, board::Board}, evals::{cell_score::CellEval, EvalTrait}};

pub struct MinimaxAgent {
    pub depth: i32,
}

impl Agent for MinimaxAgent {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        let mut best_action: Option<Action> = None;
        let mut best_score: ScoreType = -INF;
        for action in board.legal_actions() {
            let next_board: Board = board.play_onestep(action);
            let score: ScoreType = -MinimaxAgent::minimax_score(&next_board, self.depth);
            if score > best_score {
                best_score = score;
                best_action = Some(action);
            }
        }
        best_action
    }
}

impl MinimaxAgent {
    fn minimax_score(board: &Board, depth: i32) -> ScoreType {
        if board.status() == BoardStatus::Finished || depth == 0 {
            return CellEval::eval(*board);
        }
        let legal_actions: Vec<Action> = board.legal_actions();
        if legal_actions.is_empty() {
            match board.status() {
                BoardStatus::Finished => { return CellEval::eval(*board); },
                BoardStatus::Pass => {
                    let next_board: Board = (*board).play_pass();
                    return -MinimaxAgent::minimax_score(&next_board, depth);
                },
                _ => {},
            }
        }
        let mut best_score: ScoreType = -INF;
        for action in legal_actions {
            let next_board: Board = (*board).play_onestep(action);
            let score: ScoreType = -MinimaxAgent::minimax_score(&next_board, depth-1);
            if score > best_score {
                best_score = score;
            }
        }
        best_score
    }
}
