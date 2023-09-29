use std::io;
use std::time::Instant;


macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn read_turn_id() -> i32 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let id = parse_input!(input_line, i32); // id of your player. 0: first, 1: second
    id
}

fn read_board_len() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _ = parse_input!(input_line, i32); // 8
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
}


fn main() {
    let id = read_turn_id();
    read_board_len();
    let turn: Turn = if id == 0 {
        Turn::Black
    } else {
        Turn::White
    };

    // ---Select-Strategy-Here-------
    let agent = MCTS{expand_threshold: 13, rest_time: 10};
    // ---Select-Strategy-Here-------

    loop {
        let state: State = State::build_from_text(turn);
        read_action();
        println!("{}", agent.get_action_from(state).to_string()); // a-h1-8
    }
}



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

    pub fn winning_status(&self) -> WinningStatus {
        if self.status() == BoardStatus::Finished {
            let player_cnt = self.state.player_bit.count();
            let opponent_cnt = self.state.opponent_bit.count();
            if player_cnt > opponent_cnt {
                WinningStatus::Win
            } else if player_cnt < opponent_cnt {
                WinningStatus::Lose
            } else {
                WinningStatus::Draw
            }
        } else {
            WinningStatus::NotFinished
        }
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

    pub fn to_string(&self) -> String {
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

pub type BitBoard = u64;
pub const TOP_BIT: u64 = 0x8000000000000000;
pub const BOARD_LEN: usize = 8;
pub const BOARD_SIZE: usize = 64;
pub const FIRST_BLACK_BIT: BitBoard = 0x0000000810000000;
pub const FIRST_WHITE_BIT: BitBoard = 0x0000001008000000;
pub const MAX_ACTION_NUM: usize = 33; // // オセロの合法手の最大値は33らしい. https://eukaryote.hateblo.jp/entry/2023/05/17/163629

pub type ScoreType = i32;
pub const INF: ScoreType = 10000;

pub const TIME_LIMT:u128  = 150000; // micro sec

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


#[derive(PartialEq)]
pub enum BoardStatus {
    Usual,
    Pass,
    Finished,
}


#[derive(PartialEq)]
pub enum WinningStatus {
    Win,
    Lose,
    Draw,
    NotFinished,
}


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


#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Turn {
    Black,
    White
}

pub trait TurnTrait {
    fn reverse(&self) -> Turn;
}
pub const FIRST_TURN: Turn = Turn::Black;

impl TurnTrait for Turn {
    fn reverse(&self) -> Turn {
        if *self == Turn::Black {
            Turn::White
        } else {
            Turn::Black
        }
    }
}



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


type ValueType = f64;

const INF_VALUE: ValueType = 10000.0;

struct Node<'a> {
    pub board: Board<'a>,
    pub sum_w:  ValueType,
    pub child_nodes:    Vec<Node<'a>>,
    pub try_count:  u32,
}

impl<'a> Node<'a> {
    pub fn init(board: Board<'a>) -> Node<'a> {
        Node {
            board,
            sum_w:  0.,
            child_nodes:    vec![],
            try_count:  0,
        }
    }

    pub fn expand(& mut self) {
        let actions: Vec<Action> = self.board.legal_actions();
        for action in actions {
            let new_board = self.board.play_onestep(action);
            let add_node: Node = Node::init(new_board);
            self.child_nodes.push(add_node);
        }
    }

    pub fn evaluate(&mut self, expand_threshold: u32) -> ValueType {
        let winning_status = self.board.winning_status();
        if winning_status != WinningStatus::NotFinished {
            let value:ValueType = if winning_status == WinningStatus::Win {
                1.
            } else if winning_status == WinningStatus::Lose {
                0.
            } else {
                0.5
            };
            self.sum_w += value;
            self.try_count += 1;
            value
        } else if self.child_nodes.is_empty() {
            let value = self.get_playout_score();
            self.sum_w += value;
            self.try_count += 1;
            if self.try_count == expand_threshold {
                self.expand();
            }
            value
        } else {
            let index = self.next_child_node_index();
            let node = &mut self.child_nodes[index];
            let value = 1. - node.evaluate(expand_threshold);
            self.sum_w += value;
            self.try_count += 1;
            value
        }
    }

    fn get_playout_score(&self) -> ValueType {
        let winning_status = self.board.winning_status();
        if winning_status == WinningStatus::Win {
            1.
        } else if winning_status == WinningStatus::Lose {
            0.
        } else {
            0.5
        }
    }

    fn next_child_node_index(&self) -> usize {
        let mut t = 0;
        for (i, node) in self.child_nodes.iter().enumerate() {
            if node.try_count == 0 {
                return i;
            }
            t += node.try_count;
        }
        let mut best_value = -INF_VALUE;
        let mut best_index = 0;

        for (i, node) in self.child_nodes.iter().enumerate() {
            let ucb1_value: f64 = 1. - node.sum_w / node.try_count as ValueType
                                + 1. * (2. * (t as ValueType).ln() / node.try_count as ValueType).sqrt();
            if ucb1_value > best_value {
                best_index = i;
                best_value = ucb1_value;
            }
        }
        best_index
    }
}

pub struct MCTS {
    pub expand_threshold:  u32,
    pub rest_time: u128, // micro sec
}

impl Agent for MCTS {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        let now = Instant::now();
        let mut root_node: Node = Node::init(*board);
        root_node.expand();
        while now.elapsed().as_micros() + self.rest_time < TIME_LIMT {
            root_node.evaluate(self.expand_threshold);
        }
        let actions = board.legal_actions();
        let mut res_action = None;

        let mut most_try_count = 0;
        for i in 0..actions.len() {
            let count = root_node.child_nodes[i].try_count;
            if count > most_try_count {
                most_try_count = count;
                res_action = Some(actions[i]);
            }
        }

        res_action
    }
}