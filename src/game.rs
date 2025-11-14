mod board;
mod bot;
mod movement;
mod player;
mod win_checker;

use self::board::Board;
use self::bot::Bot;
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
    bot: Option<Bot>,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        self.set_board_size();
        self.set_default_players();
        self.enable_bot_for_player_2();
        self.set_default_current_player();
        self.set_default_movements();
        self.draw_board();

        while !win_checker::game_over(&self.movements) {
            self.play_single_turn();
        }

        println!(
            "Game over! Player '{}' wins!",
            self.movements.get_last_player().get_symbol()
        );
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

    pub fn enable_bot_for_player_2(&mut self) {
        self.bot = Some(Bot::new(
            self.player_none.get_symbol().to_string(),
            self.player_2.get_symbol().to_string(),
            self.player_1.get_symbol().to_string(),
        ));
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

    fn switch_player(&mut self) {
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
        if self.is_current_player_bot() {
            self.make_bot_move();
        } else {
            self.input_movement_until_valid();
        }
        self.draw_board();
        self.switch_player();
    }

    fn is_current_player_bot(&self) -> bool {
        !self.is_player_1_turn && self.bot.is_some()
    }

    fn make_bot_move(&mut self) {
        if let Some(bot) = &self.bot {
            let current_player = self.get_current_player();
            let (x, y) = bot.make_movement(&self.movements);

            if let Err(error) = self.movements.add(x, y, Rc::clone(current_player)) {
                eprintln!("[Error] Bot move failed: {}", error);
            }
        }
    }

    fn input_movement_until_valid(&mut self) {
        loop {
            let current_player = self.get_current_player();
            let (x, y) = util::input_movement(
                self.board_width,
                self.board_height,
                current_player.get_symbol(),
            );

            if let Err(error) = self.movements.add(x - 1, y - 1, Rc::clone(current_player)) {
                eprintln!("[Error] {}", error);
                continue;
            }
            break;
        }
    }
}
