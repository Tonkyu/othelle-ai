use rand::seq::SliceRandom;

use super::Agent;
use crate::components::{action::Action, state::State};

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn next_action(&self, state: &State) -> Action {
        state.legal_actions().choose(&mut rand::thread_rng()).unwrap().clone()
    }
}
