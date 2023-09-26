mod components;
use components::board::Board;

fn main() {
    let mut board: Board = Board::init();
    board.print();
    board.play("D3");
    board.print();
}