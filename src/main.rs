mod components;
mod agents;
use agents::random::RandomAgent;
use components::turn::{BLACK_TURN, WHITE_TURN};


use crate::agents::Agent;
use crate::components::state::State;
use crate::components::turn::Turn;

use std::io;
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn read_action() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let action_count = parse_input!(input_line, i32); // number of legal actions for this turn.

    for _ in 0..action_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _ = input_line.trim().to_string(); // the action
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _ = parse_input!(input_line, i32); // 8
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let id = parse_input!(input_line, i32); // id of your player. 0: first, 1: second
    let turn: Turn = if id == 0 {
        BLACK_TURN
    } else {
        WHITE_TURN
    };

    let agent = RandomAgent{};
    loop {
        let state: State = State::build_from_text(turn);
        read_action();
        println!("{}", agent.next_action(&state).to_str()); // a-h1-8
    }
}
