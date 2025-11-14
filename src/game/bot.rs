use super::movement::Movement;
use super::player::Player;
use rand::Rng;
use std::rc::Rc;

pub struct Bot {
    player_none_symbol: String,
    player_bot_symbol: String,
    player_opponent_symbol: String,
}

impl Bot {
    pub fn new(
        player_none_symbol: String,
        player_bot_symbol: String,
        player_opponent_symbol: String,
    ) -> Self {
        Self {
            player_none_symbol,
            player_bot_symbol,
            player_opponent_symbol,
        }
    }

    pub fn make_movement(&self, movements: &Movement) -> (usize, usize) {
        let available_movements = movements.get_available_movements();
        let mut cloned_movements = self.clone_movements(movements);

        if let Some(winning_movement) =
            self.find_winning_movement(&mut cloned_movements, &available_movements)
        {
            return winning_movement;
        }

        if let Some(blocking_movement) =
            self.find_blocking_movement(&mut cloned_movements, &available_movements)
        {
            return blocking_movement;
        }

        self.make_random_movement(&available_movements)
    }

    fn make_random_movement(&self, available_movements: &Vec<(usize, usize)>) -> (usize, usize) {
        let random_index = rand::rng().random_range(0..available_movements.len());
        available_movements[random_index]
    }

    fn clone_movements(&self, movements: &Movement) -> Movement {
        let width = movements.width();
        let height = movements.height();

        let player_none = Rc::new(Player::new(self.player_none_symbol.as_str()));
        let player_bot = Rc::new(Player::new(self.player_bot_symbol.as_str()));
        let player_opponent = Rc::new(Player::new(self.player_opponent_symbol.as_str()));

        let mut cloned = Movement::new(width, height, Rc::clone(&player_none));

        for x in 0..width {
            for y in 0..height {
                let original_player_symbol = movements.get_player_at(x, y).get_symbol();

                if original_player_symbol == self.player_none_symbol {
                    let _ = cloned.add(x, y, Rc::clone(&player_none));
                } else if original_player_symbol == self.player_bot_symbol {
                    let _ = cloned.add(x, y, Rc::clone(&player_bot));
                } else if original_player_symbol == self.player_opponent_symbol {
                    let _ = cloned.add(x, y, Rc::clone(&player_opponent));
                }
            }
        }

        cloned
    }

    fn find_winning_movement(
        &self,
        cloned_movements: &mut Movement,
        available_movements: &Vec<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        for (x, y) in available_movements {
            if self.would_win(cloned_movements, *x, *y) {
                return Some((*x, *y));
            }
        }

        None
    }

    fn would_win(&self, cloned_movements: &mut Movement, x: usize, y: usize) -> bool {
        let player_bot = Rc::new(Player::new(&self.player_bot_symbol));

        if cloned_movements.add(x, y, player_bot).is_ok() {
            if super::win_checker::game_over(&cloned_movements) {
                return true;
            } else {
                cloned_movements.remove_last_movement();
            }
        }

        false
    }

    fn find_blocking_movement(
        &self,
        cloned_movements: &mut Movement,
        available_movements: &Vec<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        for (x, y) in available_movements {
            if self.would_opponent_win(cloned_movements, *x, *y) {
                return Some((*x, *y));
            }
        }

        None
    }

    fn would_opponent_win(&self, cloned_movements: &mut Movement, x: usize, y: usize) -> bool {
        let player_opponent = Rc::new(Player::new(&self.player_opponent_symbol));

        if cloned_movements.add(x, y, player_opponent).is_ok() {
            if super::win_checker::game_over(&cloned_movements) {
                return true;
            } else {
                cloned_movements.remove_last_movement();
            }
        }

        false
    }
}
