use rand::seq::SliceRandom;

use super::Agent;
use crate::components::{action::Action, board::Board};

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        board.legal_actions().choose(&mut rand::thread_rng()).map(|x| x.clone())
    }
}
