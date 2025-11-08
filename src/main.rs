use ttt5::Board;

fn main() {
    let width: i32 = 15;
    let height: i32 = 10;
    let board = Board::new(width, height);

    board.draw();
}
