use super::player::Player;
use std::rc::Rc;

pub struct Movement {
    movements: Vec<Vec<Rc<Player>>>,
}

impl Movement {
    pub fn new(width: usize, height: usize, player_none: Rc<Player>) -> Self {
        Self {
            movements: vec![vec![player_none; height]; width],
        }
    }

    pub fn add(&mut self, x: usize, y: usize, player: Rc<Player>) {
        self.movements[x - 1][y - 1] = player;
    }

    pub fn get_player_at(&self, x: usize, y: usize) -> &Player {
        &self.movements[x][y]
    }
}
