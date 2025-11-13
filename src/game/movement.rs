use super::player::Player;
use std::rc::Rc;

#[derive(Default)]
pub struct Movement {
    player_none: Rc<Player>,
    movements: Vec<Vec<Rc<Player>>>,
    last_x: usize,
    last_y: usize,
}

impl Movement {
    pub fn new(width: usize, height: usize, player_none: Rc<Player>) -> Self {
        Self {
            player_none: Rc::clone(&player_none),
            movements: vec![vec![player_none; height]; width],
            last_x: 0,
            last_y: 0,
        }
    }

    pub fn add(&mut self, x: usize, y: usize, player: Rc<Player>) -> Result<(), String> {
        if !self.is_cell_empty(x, y) {
            return Err(format!("Cell at ({}, {}) is not empty.", x + 1, y + 1));
        }

        self.movements[x][y] = player;
        self.last_x = x;
        self.last_y = y;
        Ok(())
    }

    pub fn get_player_none(&self) -> &Player {
        &self.player_none
    }

    pub fn get_player_at(&self, x: usize, y: usize) -> &Player {
        &self.movements[x][y]
    }

    pub fn get_last_player(&self) -> &Player {
        &self.movements[self.last_x][self.last_y]
    }

    fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        &self.movements[x][y] == &self.player_none
    }

    pub fn last_x(&self) -> usize {
        self.last_x
    }

    pub fn last_y(&self) -> usize {
        self.last_y
    }

    pub fn width(&self) -> usize {
        self.movements.len()
    }

    pub fn height(&self) -> usize {
        self.movements[0].len()
    }
}
