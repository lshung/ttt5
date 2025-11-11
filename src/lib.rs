mod board;
mod movement;
mod player;
mod util;

use self::board::Board;
use self::movement::Movement;
use self::player::Player;
use std::rc::Rc;

pub fn run() {
    let width: usize = util::input_board_width();
    let height: usize = util::input_board_height();

    let player_1 = Rc::new(Player::new("X"));
    let player_2 = Rc::new(Player::new("O"));
    let player_none = Rc::new(Player::new(" "));

    let mut movements = Movement::new(width, height, Rc::clone(&player_none));

    let mut current_player = &player_1;
    let mut i: usize = 1;

    while i <= 5 {
        movements.add(i, i, Rc::clone(current_player));
        let board = Board::new(width, height, &movements);

        util::clear_screen();
        board.draw();

        current_player = if current_player == &player_1 {
            &player_2
        } else {
            &player_1
        };
        i += 1;
    }

    println!("Game over!");
}
