mod board;
mod movement;
mod player;

use self::board::Board;
use self::movement::Movement;
use self::player::Player;
use crate::util;
use std::rc::Rc;

#[derive(Default)]
pub struct Game {
    board_width: usize,
    board_height: usize,
    player_1: Rc<Player>,
    player_2: Rc<Player>,
    player_none: Rc<Player>,
    is_player_1_turn: bool,
    movements: Movement,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        self.set_board_size();
        self.set_default_players();
        self.set_default_current_player();
        self.set_default_movements();
        self.draw_board();

        let mut i: usize = 1;
        while i <= 5 {
            self.play_single_turn();
            i += 1;
        }

        println!("Game over!");
    }

    fn set_board_size(&mut self) {
        self.board_width = util::input_board_width();
        self.board_height = util::input_board_height();
    }

    fn set_default_players(&mut self) {
        self.player_1 = Rc::new(Player::new("X"));
        self.player_2 = Rc::new(Player::new("O"));
        self.player_none = Rc::new(Player::new(" "));
    }

    fn set_default_current_player(&mut self) {
        self.is_player_1_turn = true;
    }

    fn get_current_player(&self) -> &Rc<Player> {
        if self.is_player_1_turn {
            &self.player_1
        } else {
            &self.player_2
        }
    }

    fn switch_current_player(&mut self) {
        self.is_player_1_turn = !self.is_player_1_turn;
    }

    fn set_default_movements(&mut self) {
        self.movements = Movement::new(
            self.board_width,
            self.board_height,
            Rc::clone(&self.player_none),
        );
    }

    fn draw_board(&self) {
        let board = Board::new(self.board_width, self.board_height, &self.movements);
        util::clear_screen();
        board.draw();
    }

    fn play_single_turn(&mut self) {
        self.input_movement_until_valid();
        self.draw_board();
        self.switch_current_player();
    }

    fn input_movement_until_valid(&mut self) {
        loop {
            let current_player = self.get_current_player();
            let (x, y) = util::input_movement(
                self.board_width,
                self.board_height,
                current_player.get_symbol(),
            );

            if let Err(error) = self.movements.add(x, y, Rc::clone(current_player)) {
                eprintln!("[Error] {}", error);
                continue;
            }
            break;
        }
    }
}
