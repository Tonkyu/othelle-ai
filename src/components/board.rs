use super::{state::State, action::Action, turn::{Turn, TurnTrait, BLACK_TURN, FIRST_TURN, WHITE_TURN}, bitboard::{BitBoard, BitBoardTrait}, constants::TOP_BIT};


// https://qiita.com/sensuikan1973/items/459b3e11d91f3cb37e43
pub struct Board {
    turn:   Turn,
    index:  i32,
    state:  State,
}

impl Board {
    pub fn init() -> Board {
        Board {
            turn:   FIRST_TURN,
            index:  1,
            state:  State::init(),
        }
    }

    fn update_state(&mut self, action: Action) {
        let mut reverse_board: BitBoard = 0;
        for k in 0..8 {
            let mut tmp_reverse_board: BitBoard = 0;
            let mut mask: BitBoard = Board::transfer(action.bitboard, k);
            while mask != 0 && (mask & self.state.opponent_bit) != 0 {
                tmp_reverse_board |= mask;
                mask = Board::transfer(mask, k);
            }
            if mask & self.state.player_bit != 0 {
                reverse_board |= tmp_reverse_board;
            }
        }

        let new_state: State = State {
            player_bit: self.state.player_bit ^ (action.bitboard | reverse_board),
            opponent_bit: self.state.opponent_bit ^ reverse_board,
        };

        self.state = new_state;
        self.index += 1;
    }

    fn transfer(bit: BitBoard, k: i32) -> BitBoard {
        match k {
            0 => (bit << 8) & 0xffffffffffffff00, // 上
            1 => (bit << 7) & 0x7f7f7f7f7f7f7f00, // 右上
            2 => (bit >> 1) & 0x7f7f7f7f7f7f7f7f, // 右
            3 => (bit >> 9) & 0x007f7f7f7f7f7f7f, // 右下
            4 => (bit >> 8) & 0x00ffffffffffffff, // 下
            5 => (bit >> 7) & 0x00fefefefefefefe, // 左下
            6 => (bit << 1) & 0xfefefefefefefefe, // 左
            7 => (bit << 9) & 0xfefefefefefefe00, // 左上
            _ => panic!("undefined k was given in Board.transfer: k is {}", k)
        }
    }

    fn is_pass(&self) -> bool{
        let player_legal_actions_board: BitBoard = self.state.legal_actions_bitboard();
        let tmp_state: State = State {
            player_bit: self.state.opponent_bit,
            opponent_bit: self.state.player_bit,
        };
        let opponent_legal_actions_board: BitBoard = tmp_state.legal_actions_bitboard();

        // 先手番のみ置く場所がない
        player_legal_actions_board == 0 && opponent_legal_actions_board != 0
    }

    fn is_finished(&self) -> bool {
        let player_legal_actions_board: BitBoard = self.state.legal_actions_bitboard();
        let tmp_state: State = State {
            player_bit: self.state.opponent_bit,
            opponent_bit: self.state.player_bit,
        };
        let opponent_legal_actions_board: BitBoard = tmp_state.legal_actions_bitboard();

        // 両手番とも置く場所がない
        player_legal_actions_board == 0 && opponent_legal_actions_board == 0
    }

    fn swap_turn(&mut self) {
        self.turn.swap();
        self.state = State {
            player_bit: self.state.opponent_bit,
            opponent_bit: self.state.player_bit,
        };
    }

    fn result(&self) -> (i32, i32, &str) {
        let black_score: i32;
        let white_score: i32;

        if self.turn == BLACK_TURN {
            black_score = self.state.player_bit.count();
            white_score = self.state.opponent_bit.count();
        } else {
            white_score = self.state.player_bit.count();
            black_score = self.state.opponent_bit.count();
        }

        let winner: &str = if black_score > white_score {
            "Black"
        } else if black_score < white_score {
            "White"
        } else {
            "Draw"
        };

        (black_score, white_score, winner)
    }

    pub fn play(&mut self, action_str: &str) {
        let action: Action = Action::action_from_str(action_str);
        self.update_state(action);
    }

    pub fn print(&self) {
        println!("index:\t{}", {self.index});
        let mut mask: u64 = TOP_BIT;
        for i in 0..64 {
            let black_state = if self.turn == BLACK_TURN { self.state.player_bit } else { self.state.opponent_bit };
            let white_state = if self.turn == WHITE_TURN { self.state.player_bit } else { self.state.opponent_bit };

            if mask & black_state != 0 {
                print!("o");
            } else if mask & white_state != 0 {
                print!("x");
            } else {
                print!(".");
            }
            mask >>= 1;

            if i % 8 == 7 {
                println!("");
            }
        }

        let black_score = if self.turn == BLACK_TURN { self.state.player_bit.count() } else { self.state.opponent_bit.count() };
        let white_score = if self.turn == WHITE_TURN { self.state.player_bit.count() } else { self.state.opponent_bit.count() };

        println!("Black:\t{}", black_score);
        println!("White:\t{}\n", white_score);

    }
}