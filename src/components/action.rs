use super::{bitboard::BitBoard, constants::{TOP_BIT, BOARD_SIZE}};

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