use super::player::Player;
use std::rc::Rc;

pub struct Movement {
    player_none: Rc<Player>,
    movements: Vec<Vec<Rc<Player>>>,
}

impl Movement {
    pub fn new(width: usize, height: usize, player_none: Rc<Player>) -> Self {
        Self {
            player_none: Rc::clone(&player_none),
            movements: vec![vec![player_none; height]; width],
        }
    }

    pub fn add(&mut self, x: usize, y: usize, player: Rc<Player>) -> Result<(), String> {
        if !self.is_cell_empty(x - 1, y - 1) {
            return Err(format!("Cell at ({}, {}) is not empty.", x, y));
        }

        self.movements[x - 1][y - 1] = player;
        Ok(())
    }

    pub fn get_player_at(&self, x: usize, y: usize) -> &Player {
        &self.movements[x][y]
    }

    fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        &self.movements[x][y] == &self.player_none
    }
}
