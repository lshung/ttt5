use super::movement::Movement;

const WIN_LENGTH: usize = 5;
const DIRECTIONS: [(isize, isize); 4] = [
    (0, 1),  // down
    (1, 0),  // right
    (1, 1),  // down-right
    (1, -1), // down-left
];

pub fn game_over(movements: &Movement) -> bool {
    if movements.get_last_player() == movements.get_player_none() {
        return false;
    }

    for &(dx, dy) in &DIRECTIONS {
        let forward_count = count_in_direction(movements, dx, dy);
        let backward_count = count_in_direction(movements, -dx, -dy);
        let total_count = 1 + forward_count + backward_count;

        if total_count >= WIN_LENGTH {
            return true;
        }
    }

    false
}

fn count_in_direction(movements: &Movement, dx: isize, dy: isize) -> usize {
    let width = movements.width();
    let height = movements.height();
    let last_player = movements.get_last_player();

    let mut count = 0;
    let mut current_x = movements.last_x();
    let mut current_y = movements.last_y();

    loop {
        let next_x = step_coordinate(current_x, dx, width);
        let next_y = step_coordinate(current_y, dy, height);

        let (Some(nx), Some(ny)) = (next_x, next_y) else {
            break;
        };

        if movements.get_player_at(nx, ny).get_symbol() == last_player.get_symbol() {
            count += 1;
            current_x = nx;
            current_y = ny;
        } else {
            break;
        }
    }

    count
}

fn step_coordinate(current: usize, delta: isize, limit: usize) -> Option<usize> {
    current
        .checked_add_signed(delta)
        .filter(|&next| next < limit)
}
