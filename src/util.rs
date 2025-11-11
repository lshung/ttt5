mod input_prompt;
mod validator;

use self::input_prompt::InputPrompt;
use self::validator::Validators;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn input_board_width() -> usize {
    InputPrompt::new("Enter the width of the board: ")
        .with_validator(Validators::non_empty())
        .with_validator(Validators::usize_range(5, 99))
        .get_usize_number()
}

pub fn input_board_height() -> usize {
    InputPrompt::new("Enter the height of the board: ")
        .with_validator(Validators::non_empty())
        .with_validator(Validators::usize_range(5, 99))
        .get_usize_number()
}

pub fn input_movement(
    board_width: usize,
    board_height: usize,
    player_symbol: &str,
) -> (usize, usize) {
    loop {
        let prompt = format!("Enter the movement (x y) for Player '{}': ", player_symbol);
        let (x, y) = InputPrompt::new(prompt.as_str())
            .with_validator(Validators::non_empty())
            .with_validator(Validators::has_two_usize_numbers())
            .get_two_usize_numbers();

        if x < 1 || x > board_width {
            eprintln!(
                "[Error] Invalid x value. Please enter a value between 1 and {}.",
                board_width
            );
            continue;
        }

        if y < 1 || y > board_height {
            eprintln!(
                "[Error] Invalid y value. Please enter a value between 1 and {}.",
                board_height
            );
            continue;
        }

        return (x, y);
    }
}
