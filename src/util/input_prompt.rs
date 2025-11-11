use std::io::{self, Write};

pub struct InputPrompt {
    prompt: String,
    validators: Vec<Box<dyn Fn(&str) -> Result<(), String>>>,
}

impl InputPrompt {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            validators: Vec::new(),
        }
    }

    pub fn with_validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> Result<(), String> + 'static,
    {
        self.validators.push(Box::new(validator));
        self
    }

    pub fn get_text(self) -> String {
        loop {
            match self.read_input() {
                Ok(input) => return input,
                Err(error) => Self::display_error(&error),
            }
        }
    }

    pub fn get_usize_number(self) -> usize {
        loop {
            match self.read_input() {
                Ok(input) => match input.parse::<usize>() {
                    Ok(value) => return value,
                    Err(_) => Self::display_error("Please enter a valid usize number."),
                },
                Err(error) => Self::display_error(&error),
            }
        }
    }

    fn read_input(&self) -> Result<String, String> {
        Self::display_prompt(&self.prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}.", e))?;

        let input = input.trim().to_string();

        for validator in &self.validators {
            validator(&input).map_err(|e| e)?;
        }

        Ok(input)
    }

    fn display_prompt(prompt: &str) {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout.");
    }

    fn display_error(error: &str) {
        eprintln!("Error: {}", error);
    }
}
