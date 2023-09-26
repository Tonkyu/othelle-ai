use super::{action::Action, bitboard::BitBoard, constants::{FIRST_WHITE_BIT, FIRST_BLACK_BIT}};

pub struct State {
    pub player_bit:  BitBoard,
    pub opponent_bit: BitBoard,
}

impl State {
    pub fn init() -> State {
        let player_bit: BitBoard = FIRST_BLACK_BIT;
        let opponent_bit: BitBoard = FIRST_WHITE_BIT;
        State::build(player_bit, opponent_bit)
    }

    pub fn build(player_bit: BitBoard, opponent_bit: BitBoard) -> State {
        State {
            player_bit,
            opponent_bit
        }
    }

    fn is_legal_action(&self, action: Action) -> bool {
        let legal_bitboard: BitBoard = self.legal_actions_bitboard();
        legal_bitboard & action.bitboard != 0
    }
}

// legal_actions_bitboardのみで使う.高速化のために外に出しておく(効果未検証)
struct Direction {
    watch_board: BitBoard,
    shift_step: i32,
}

impl State {
    pub fn legal_actions_bitboard(&self) -> BitBoard { // 着手可能なマスにフラグが立っている
        let horizontal_watch_board: BitBoard = self.opponent_bit & 0x7e7e7e7e7e7e7e7e;
        let vertical_watch_board: BitBoard = self.opponent_bit & 0x00FFFFFFFFFFFF00;
        let all_side_watch_board: BitBoard = self.opponent_bit & 0x007e7e7e7e7e7e00;

        let blank_board: BitBoard = !(self.player_bit | self.opponent_bit);

        let mut legal_board: BitBoard = 0;
        let directions: [Direction; 8];

        directions = [
            Direction {
                watch_board: horizontal_watch_board,
                shift_step: 1,
            },
            Direction {
                watch_board: horizontal_watch_board,
                shift_step: -1,
            },
            Direction {
                watch_board: vertical_watch_board,
                shift_step: 8,
            },
            Direction {
                watch_board: vertical_watch_board,
                shift_step: -8,
            },
            Direction {
                watch_board: all_side_watch_board,
                shift_step: 7,
            },
            Direction {
                watch_board: all_side_watch_board,
                shift_step: -7,
            },
            Direction {
                watch_board: all_side_watch_board,
                shift_step: 9,
            },
            Direction {
                watch_board: all_side_watch_board,
                shift_step: -9,
            }
        ];

        for dir in directions.iter() {
            let mut tmp_board: BitBoard = 0;
            for j in 0..6 {
                tmp_board |= dir.watch_board & (tmp_board << dir.shift_step);
            }
            legal_board |= blank_board & (tmp_board << dir.shift_step);
        }

        legal_board
    }
}