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
