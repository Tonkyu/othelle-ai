use super::components::{state::State, action::Action};

pub mod random;
pub mod human;

pub trait Agent {
    fn next_action(&self, state: &State) -> Action;
}