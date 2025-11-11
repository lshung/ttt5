pub struct Validators;

impl Validators {
    pub fn non_empty() -> impl Fn(&str) -> Result<(), String> {
        |input: &str| {
            if input.is_empty() {
                Err("This field cannot be empty.".into())
            } else {
                Ok(())
            }
        }
    }

    pub fn usize_range(min: usize, max: usize) -> impl Fn(&str) -> Result<(), String> {
        move |input: &str| match input.parse::<usize>() {
            Ok(num) if num >= min && num <= max => Ok(()),
            Ok(_) => Err(format!("Must be between {} and {}.", min, max)),
            Err(_) => Err("Please enter a valid number.".into()),
        }
    }
}
