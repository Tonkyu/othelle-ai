use std::io;
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn read_action() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let action_count = parse_input!(input_line, i32); // number of legal actions for this turn.

    for i in 0..action_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let action = input_line.trim().to_string(); // the action
    }
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


    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let board_size = parse_input!(input_line, i32); // 8


    let agent = RandomAgent{};
    loop {
        let state: State = State::build_from_text(turn);
        read_action();
        let bit = state.legal_actions_bitboard();
        println!("{} MSG {:016x}", agent.next_action(&state).to_str(), bit); // a-h1-8
    }
}



pub type Turn = bool;

pub const BLACK_TURN: Turn = true;
pub const WHITE_TURN: Turn = false;
pub const FIRST_TURN: Turn = BLACK_TURN;

pub trait TurnTrait {
    fn swap(&mut self);
}

impl TurnTrait for Turn {
    fn swap(&mut self) {
        if *self == BLACK_TURN {
            *self = WHITE_TURN;
        } else {
            *self = BLACK_TURN;
        }
    }
}





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



pub type BitBoard = u64;

pub trait BitBoardTrait {
    fn count(&self) -> i32;
}

impl BitBoardTrait for BitBoard {
    fn count(&self) -> i32 {
        let mut mask: BitBoard = TOP_BIT;
        let mut count = 0;

        for _ in 0..BOARD_SIZE {
            if mask & self != 0 {
                count += 1;
            }
            mask >>= 1;
        }

        count
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Action {
    pub bitboard: BitBoard,
}

impl Action {
    pub fn action_from_bitboard(bitboard: BitBoard) -> Action{
        Action { bitboard }
    }

    pub fn action_from_str(action_str: &str)  -> Action {
        let ch_x: char = action_str.chars().nth(0).unwrap();
        let ch_y: char = action_str.chars().nth(1).unwrap();
        let num_x: i32 = (ch_x as i32) - ('a' as i32);
        let num_y: i32 = (ch_y as i32) - ('1' as i32);

        let mut mask: u64 = TOP_BIT;

        if num_x < 0 || 8 <= num_x {
            panic!("Invalid action format: x is {}", num_x)
        }

        if num_y < 0 || 8 <= num_y {
            panic!("Invalid action format: y is {}", num_y)
        }

        mask >>= num_x;
        mask >>= num_y * 8;

        Action {
            bitboard: mask,
        }
    }

    pub fn to_str(&self) -> String {
        let mut mask:BitBoard = TOP_BIT;
        let mut res: String = String::new();
        for i in 0..BOARD_SIZE {
            if self.bitboard & mask != 0 {
                res = format!("{}{}", (((i % 8) as u8 + 'a' as u8) as char), (((i / 8) as u8 + '1' as u8) as char));
                break;
            }
            mask >>= 1;
        }
        res
    }
}


pub trait Agent {
    fn next_action(&self, state: &State) -> Action;
}


use rand::seq::SliceRandom;

pub struct RandomAgent {}

impl Agent for RandomAgent {
    fn next_action(&self, state: &State) -> Action {
        state.legal_actions().choose(&mut rand::thread_rng()).unwrap().clone()
    }
}



pub const TOP_BIT: u64 = 0x8000000000000000;
pub const BOARD_LEN: usize = 8;
pub const BOARD_SIZE: usize = 64;
pub const FIRST_BLACK_BIT: BitBoard = 0x0000000810000000;
pub const FIRST_WHITE_BIT: BitBoard = 0x0000001008000000;
pub const MAX_ACTION_NUM: usize = 33; // // オセロの合法手の最大値は33らしい. https://eukaryote.hateblo.jp/entry/2023/05/17/163629