use super::constants::{TOP_BIT, BOARD_SIZE};

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