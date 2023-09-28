use super::components::{board::Board, action::Action, state::State};

pub mod random;
pub mod human;
pub mod minimax;
pub mod alphabeta;

pub trait Agent {
    fn next_action_option(&self, board: &Board) -> Option<Action>;

    fn next_action(&self, board: &Board) -> Action {
        self.next_action_option(board).expect("Not found valid action")
    }

    fn get_action_from(&self, state: State) -> Action where Self: Sized{
        let mut tmp_board = Board::init(self, self);
        tmp_board.set_state(state);
        self.next_action(&tmp_board)
    }
}