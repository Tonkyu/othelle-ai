use super::{action::Action, bitboard::BitBoard, constants::{FIRST_WHITE_BIT, FIRST_BLACK_BIT, TOP_BIT, BOARD_SIZE, MAX_ACTION_NUM, BOARD_LEN}, turn::{Turn, FIRST_TURN}};
use std::io;

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

    pub fn build_from_text(turn: Turn) -> State {
        let mut first_bit: BitBoard = 0;
        let mut second_bit: BitBoard = 0;

        for i in 0..BOARD_LEN {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let line = input_line.trim().to_string();

            for j in 0..BOARD_LEN {
                if let Some(character) = line.chars().nth(j) {
                    if character == '0' {
                        first_bit |= TOP_BIT >> (BOARD_LEN * i + j);
                    } else if character == '1' {
                        second_bit |= TOP_BIT >> (BOARD_LEN * i + j);
                    }
                }
            }
        }

        if turn == FIRST_TURN {
            State::build(first_bit, second_bit)
        } else {
            State::build(second_bit, first_bit)
        }
    }
}

// legal_actions_bitboardのみで使う.高速化のために外に出しておく(効果未検証)
struct Direction {
    watch_board: BitBoard,
    shift_step: i32,
}

impl State {
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

    pub fn legal_actions_bitboard(&self) -> BitBoard { // 着手可能なマスにフラグが立っている
        let horizontal_watch_board: BitBoard = self.opponent_bit & 0x7e7e7e7e7e7e7e7e;
        let vertical_watch_board: BitBoard = self.opponent_bit & 0x00FFFFFFFFFFFF00;
        let all_side_watch_board: BitBoard = self.opponent_bit & 0x007e7e7e7e7e7e00;

        let blank_board: BitBoard = !(self.player_bit | self.opponent_bit);

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

            tmp_board = dir.watch_board & (self.player_bit << dir.shift_step);
            for _ in 0..5 {
                tmp_board |= dir.watch_board & (tmp_board << dir.shift_step);
            }
            legal_board |= blank_board & (tmp_board << dir.shift_step);

            tmp_board = dir.watch_board & (self.player_bit >> dir.shift_step);
            for _ in 0..5 {
                tmp_board |= dir.watch_board & (tmp_board >> dir.shift_step);
            }
            legal_board |= blank_board & (tmp_board >> dir.shift_step);
        }
        legal_board
    }
}