use agents::{random::RandomAgent, minimax::MinimaxAgent, human::HumanAgent};
use components::board::Board;

mod components;
mod agents;
mod evals;

fn main() {
    let player_agent = MinimaxAgent{depth: 3};
    let opponent_agent = RandomAgent{};
    let board: Board = Board::init(&player_agent, &opponent_agent);
    board.playout();
}
