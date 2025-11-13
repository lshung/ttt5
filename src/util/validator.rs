pub struct Validator;

impl Validator {
    pub fn non_empty() -> impl Fn(&str) -> Result<(), String> {
        |input: &str| {
            if input.is_empty() {
                Err("This field cannot be empty.".to_string())
            } else {
                Ok(())
            }
        }
    }

    pub fn usize_range(min: usize, max: usize) -> impl Fn(&str) -> Result<(), String> {
        move |input: &str| match input.parse::<usize>() {
            Ok(num) if num >= min && num <= max => Ok(()),
            Ok(_) => Err(format!("Must be between {} and {}.", min, max)),
            Err(_) => Err("Please enter a valid number.".to_string()),
        }
    }

    pub fn has_two_usize_numbers() -> impl Fn(&str) -> Result<(), String> {
        move |input: &str| {
            let parts: Vec<&str> = input.split(" ").collect();

            if parts.len() != 2 {
                return Err("Please enter two usize numbers separated by a space.".to_string());
            }

            if let Err(_) = parts[0].trim().parse::<usize>() {
                return Err("Invalid x value. Please enter a valid usize number.".to_string());
            }

            if let Err(_) = parts[1].trim().parse::<usize>() {
                return Err("Invalid y value. Please enter a valid usize number.".to_string());
            }

            Ok(())
        }
    }
}
