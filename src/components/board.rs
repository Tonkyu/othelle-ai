use crate::agents::Agent;

use super::{state::State, action::Action, turn::{Turn, TurnTrait, FIRST_TURN}, bitboard::{BitBoard, BitBoardTrait}, constants::{TOP_BIT, MAX_ACTION_NUM, BOARD_SIZE, BoardStatus}};


// https://qiita.com/sensuikan1973/items/459b3e11d91f3cb37e43

#[derive(Copy, Clone)]
pub struct Board<'a> {
    turn:   Turn,
    index:  i32,
    pub state:  State,
    player_agent:   &'a dyn Agent,
    opponent_agent: &'a dyn Agent,
}

impl<'a> Board<'a> {
    pub fn init(player_agent: &'a dyn Agent, opponent_agent: &'a dyn Agent) -> Board<'a> {
        Board {
            turn:   FIRST_TURN,
            index:  1,
            state:  State::init(),
            player_agent,
            opponent_agent,
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
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

    pub fn status(&self) -> BoardStatus {
        let tmp_board: Board = Board {
            state: State {
                player_bit: self.state.opponent_bit,
                opponent_bit: self.state.player_bit,
            },
            ..*self
        };

        let player_legal_actions_board: BitBoard = self.legal_actions_bitboard();
        let opponent_legal_actions_board: BitBoard = tmp_board.legal_actions_bitboard();

        if player_legal_actions_board == 0 && opponent_legal_actions_board != 0 {  // 先手番のみ置く場所がない
            BoardStatus::Pass
        } else if player_legal_actions_board == 0 && opponent_legal_actions_board == 0 { // 両手番とも置く場所がない
            BoardStatus::Finished
        } else {
            BoardStatus::Usual
        }
    }

    fn result(&self) -> (i32, i32, &str) {
        let black_score: i32;
        let white_score: i32;

        if self.turn == Turn::Black {
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

    pub fn play_onestep(self, action: Action) -> Board<'a> {
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

        let res_board = Board {
            state:  State {
                player_bit:     self.state.opponent_bit ^ reverse_board,
                opponent_bit:   self.state.player_bit ^ (action.bitboard | reverse_board),
            },
            index:  self.index + 1,
            turn:   self.turn.reverse(),
            player_agent:   self.opponent_agent,
            opponent_agent: self.player_agent,
        };
        res_board
    }

    pub fn play_pass(self) -> Board<'a> {
        Board {
            state:  State {
                player_bit:     self.state.opponent_bit,
                opponent_bit:   self.state.player_bit,
            },
            index:  self.index + 1,
            turn:   self.turn.reverse(),
            player_agent:   self.opponent_agent,
            opponent_agent: self.player_agent,
        }
    }

    pub fn playout(self) {
        let mut tmp: Board = self;
        let mut tmp_status: BoardStatus = self.status();
        while tmp_status != BoardStatus::Finished {
            tmp.print();
            if tmp_status == BoardStatus::Pass {
                tmp = tmp.play_pass();
            } else {
                let action: Action = tmp.player_agent.next_action(&tmp);
                tmp = tmp.play_onestep(action);
            }
            tmp_status = tmp.status();
        }
        let result = tmp.result();

        tmp.print();
        println!("Result:{}", result.2);
    }

    pub fn print(&self) {
        println!("index:\t{}", self.index);
        println!("*ABCDEFGH*");
        let mut mask: u64 = TOP_BIT;
        for i in 0..64 {
            if i % 8 == 0 {
                print!("{}", i / 8 + 1);
            }
            let black_state = if self.turn == Turn::Black { self.state.player_bit } else { self.state.opponent_bit };
            let white_state = if self.turn == Turn::White { self.state.player_bit } else { self.state.opponent_bit };

            if mask & black_state != 0 {
                print!("o");
            } else if mask & white_state != 0 {
                print!("x");
            } else {
                print!(".");
            }
            mask >>= 1;

            if i % 8 == 7 {
                print!("{}\n", i / 8 + 1);
            }
        }

        let black_score = if self.turn == Turn::Black { self.state.player_bit.count() } else { self.state.opponent_bit.count() };
        let white_score = if self.turn == Turn::White { self.state.player_bit.count() } else { self.state.opponent_bit.count() };

        println!("*ABCEDFGH*");
        println!("Black:\t{}", black_score);
        println!("White:\t{}\n", white_score);

    }
}

// legal_actions_bitboardのみで使う.高速化のために外に出しておく(効果未検証)
struct Direction {
    watch_board: BitBoard,
    shift_step: i32,
}

impl<'a> Board<'a> {
    pub fn legal_actions(&self) -> Vec<Action> {
        let legal_bitboard: BitBoard = self.legal_actions_bitboard();
        let mut mask:BitBoard = TOP_BIT;
        let mut actions: Vec<Action> = Vec::with_capacity(MAX_ACTION_NUM);
        for _ in 0..BOARD_SIZE {
            if legal_bitboard & mask != 0 {
                actions.push(Action::action_from_bitboard(mask));
            }
            mask >>= 1;
        }
        actions
    }

    fn legal_actions_bitboard(&self) -> BitBoard { // 着手可能なマスにフラグが立っている
        let horizontal_watch_board: BitBoard = self.state.opponent_bit & 0x7e7e7e7e7e7e7e7e;
        let vertical_watch_board: BitBoard = self.state.opponent_bit & 0x00FFFFFFFFFFFF00;
        let all_side_watch_board: BitBoard = self.state.opponent_bit & 0x007e7e7e7e7e7e00;

        let blank_board: BitBoard = !(self.state.player_bit | self.state.opponent_bit);

        let mut legal_board: BitBoard = 0;
        let directions: [Direction; 4];

        directions = [
            Direction {
                watch_board: horizontal_watch_board,
                shift_step: 1,
            },
            Direction {
                watch_board: vertical_watch_board,
                shift_step: 8,
            },
            Direction {
                watch_board: all_side_watch_board,
                shift_step: 7,
            },
            Direction {
                watch_board: all_side_watch_board,
                shift_step: 9,
            },
        ];

        for dir in directions.iter() {
            let mut tmp_board: BitBoard;

            tmp_board = dir.watch_board & (self.state.player_bit << dir.shift_step);
            for _ in 0..5 {
                tmp_board |= dir.watch_board & (tmp_board << dir.shift_step);
            }
            legal_board |= blank_board & (tmp_board << dir.shift_step);

            tmp_board = dir.watch_board & (self.state.player_bit >> dir.shift_step);
            for _ in 0..5 {
                tmp_board |= dir.watch_board & (tmp_board >> dir.shift_step);
            }
            legal_board |= blank_board & (tmp_board >> dir.shift_step);
        }
        legal_board
    }
}