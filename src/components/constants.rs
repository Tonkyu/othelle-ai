use super::bitboard::BitBoard;

pub const TOP_BIT: u64 = 0x8000000000000000;
pub const BOARD_LEN: usize = 8;
pub const BOARD_SIZE: usize = 64;
pub const FIRST_BLACK_BIT: BitBoard = 0x0000000810000000;
pub const FIRST_WHITE_BIT: BitBoard = 0x0000001008000000;
pub const MAX_ACTION_NUM: usize = 33; // // オセロの合法手の最大値は33らしい. https://eukaryote.hateblo.jp/entry/2023/05/17/163629

pub type ScoreType = i32;
pub const INF: ScoreType = 10000;

pub const TIME_LIMT: u128 = 150000; // micro sec

#[derive(PartialEq)]
pub enum BoardStatus {
    Usual,
    Pass,
    Finished,
}
