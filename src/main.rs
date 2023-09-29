use agents::{random::RandomAgent, minimax::MiniMaxAgent, human::HumanAgent, alphabeta::AlphaBetaAgent, mcts::MCTS};
use components::{board::Board, enums::Turn};

mod components;
mod agents;
mod evals;

fn main() {
    // let mut player_agent = MinimaxAgent{depth: 3};
    let mut player_agent = RandomAgent{};
    let mut opponent_agent = MCTS{expand_threshold: 12, rest_time: 10};

    let play_time = 25;
    let mut black = 0;
    let mut white = 0;
    let mut draw = 0;
    for _ in 0..play_time {
        let board: Board = Board::init(&mut player_agent, &mut opponent_agent);
        let result = board.playout(false);
        println!("Black:\t{}\tWhite:\t{}\tResult\t{}", result.0, result.1, result.2);
        match result.2 {
            Turn::Black => { black += 1; }
            Turn::White => { white += 1; }
            Turn::Draw => { draw += 1; }
        }
    }
    println!("-----------------------------------------------------------");
    println!("Black:\t{}\tWhite:\t{}\tDraw\t{}\n", black, white, draw);

}
