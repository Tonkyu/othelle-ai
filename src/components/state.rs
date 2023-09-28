use super::{bitboard::BitBoard, constants::{FIRST_WHITE_BIT, FIRST_BLACK_BIT, TOP_BIT, BOARD_LEN}, turn::{Turn, FIRST_TURN}};
use std::io;

#[derive(Copy, Clone)]
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
