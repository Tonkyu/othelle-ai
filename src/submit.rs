use std::io;
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
    let agent = MinimaxAgent{depth: 4};
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

pub const TOP_BIT: u64 = 0x8000000000000000;
pub const BOARD_LEN: usize = 8;
pub const BOARD_SIZE: usize = 64;
pub const FIRST_BLACK_BIT: BitBoard = 0x0000000810000000;
pub const FIRST_WHITE_BIT: BitBoard = 0x0000001008000000;
pub const MAX_ACTION_NUM: usize = 33; // // オセロの合法手の最大値は33らしい. https://eukaryote.hateblo.jp/entry/2023/05/17/163629

pub type ScoreType = i32;
pub const INF: ScoreType = 10000;

#[derive(PartialEq)]
pub enum BoardStatus {
    Usual,
    Pass,
    Finished,
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




pub struct MinimaxAgent {
    pub depth: i32,
}

impl Agent for MinimaxAgent {
    fn next_action_option(&self, board: &Board) -> Option<Action> {
        let mut best_action: Option<Action> = None;
        let mut best_score: ScoreType = -INF;
        for action in board.legal_actions() {
            let next_board: Board = board.play_onestep(action);
            let score: ScoreType = -MinimaxAgent::minimax_score(&next_board, self.depth);
            if score > best_score {
                best_score = score;
                best_action = Some(action);
            }
        }
        best_action
    }
}

impl MinimaxAgent {
    fn minimax_score(board: &Board, depth: i32) -> ScoreType {
        if board.status() == BoardStatus::Finished || depth == 0 {
            return CellEval::eval(*board);
        }
        let legal_actions: Vec<Action> = board.legal_actions();
        if legal_actions.is_empty() {
            match board.status() {
                BoardStatus::Finished => { return CellEval::eval(*board); },
                BoardStatus::Pass => {
                    let next_board: Board = (*board).play_pass();
                    return -MinimaxAgent::minimax_score(&next_board, depth);
                },
                _ => {},
            }
        }
        let mut best_score: ScoreType = -INF;
        for action in legal_actions {
            let next_board: Board = (*board).play_onestep(action);
            let score: ScoreType = -MinimaxAgent::minimax_score(&next_board, depth-1);
            if score > best_score {
                best_score = score;
            }
        }
        best_score
    }
}



pub trait EvalTrait {
    fn eval(board: Board) -> ScoreType;
}

pub struct CellEval {}

impl EvalTrait for CellEval {
    fn eval(board: Board) -> ScoreType {
        let mut res_score: ScoreType = 0;
        const SCORE_TABLE: [ScoreType; BOARD_SIZE] = [
            30, -12, 0, -1, -1, 0, -12, 30,
            -12, -15, -3, -3, -3, -3, -15, -12,
            0, -3, 0, -1, -1, 0, -3, 0,
            -1, -3, -1, -1, -1, -1, -3, -1,
            -1, -3, -1, -1, -1, -1, -3, -1,
            0, -3, 0, -1, -1, 0, -3, 0,
            -12, -15, -3, -3, -3, -3, -15, -12,
            30, -12, 0, -1, -1, 0, -12, 30
        ];

        let mut mask: BitBoard = TOP_BIT;
        for i in 0..BOARD_SIZE {
            if board.state.player_bit & mask != 0 {
                res_score += SCORE_TABLE[i];
            }
            if board.state.opponent_bit & mask != 0 {
                res_score -= SCORE_TABLE[i];
            }
            mask >>= 1;
        }
        res_score
    }
}