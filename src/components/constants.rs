use super::bitboard::BitBoard;

pub const TOP_BIT: u64 = 0x8000000000000000;
pub const BOARD_SIZE: i32 = 64;
pub const FIRST_BLACK_BIT: BitBoard = 0x0000000810000000;
pub const FIRST_WHITE_BIT: BitBoard = 0x0000001008000000;