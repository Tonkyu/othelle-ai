use std::io;

use super::Agent;
use crate::components::{action::Action, state::State};

pub struct HumanAgent {}

impl Agent for HumanAgent {
    fn next_action(&self, _: &State) -> Action {
        Action::action_from_str(&HumanAgent::read_buffer())
    }
}

impl HumanAgent {
    fn read_buffer() -> String {
        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
        buffer.trim().to_string()
    }
}