mod board;
mod movement;
mod player;

use self::board::Board;
use self::movement::Movement;
use self::player::Player;
use std::rc::Rc;

pub fn run() {
    let width: usize = 15;
    let height: usize = 10;

    let player_1 = Rc::new(Player::new("X"));
    let player_2 = Rc::new(Player::new("O"));
    let player_none = Rc::new(Player::new(" "));

    let mut movements = Movement::new(width, height, Rc::clone(&player_none));
    movements.add(5, 5, Rc::clone(&player_1));
    movements.add(8, 6, Rc::clone(&player_2));
    movements.add(7, 9, Rc::clone(&player_1));
    movements.add(3, 6, Rc::clone(&player_2));

    let board = Board::new(width, height, &movements);
    board.draw();
}
