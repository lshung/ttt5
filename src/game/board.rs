use super::movement::Movement;

pub struct Board<'a> {
    width: usize,
    height: usize,
    left_padding: usize,
    movements: &'a Movement,
}

impl<'a> Board<'a> {
    pub fn new(width: usize, height: usize, movements: &'a Movement) -> Self {
        Self {
            width: width,
            height: height,
            left_padding: Self::calculate_left_padding(height),
            movements: movements,
        }
    }

    fn calculate_left_padding(height: usize) -> usize {
        match height {
            n if n < 10 => 2,
            n if n < 100 => 3,
            _ => 0,
        }
    }

    pub fn draw(&self) {
        self.print_x_axis_numbers();

        for row_index in 0..self.height {
            self.print_horizontal_border();
            self.print_single_row(row_index);
        }

        self.print_horizontal_border();
    }

    fn print_x_axis_numbers(&self) {
        print!("{}", " ".repeat(self.left_padding));
        for i in 0..self.width {
            match i {
                n if n + 1 < 10 => print!("  {} ", n + 1),
                n if n + 1 < 100 => print!(" 0{}", n + 1),
                n if n + 1 < 1000 => print!(" {}", n + 1),
                _ => panic!("Errors when printing X-axis numbers."),
            }
        }
        print!(" ");
        println!();
    }

    fn print_horizontal_border(&self) {
        print!("{}", " ".repeat(self.left_padding));
        for _ in 0..self.width {
            print!("+---");
        }
        print!("+");
        println!();
    }

    fn print_single_row(&self, row_index: usize) {
        self.print_y_axis_number(row_index);
        for i in 0..self.width {
            let player_symbol = &self.movements.get_player_at(i, row_index).get_symbol();
            print!("| {} ", player_symbol);
        }
        print!("|");
        println!();
    }

    fn print_y_axis_number(&self, row_index: usize) {
        match row_index {
            n if n + 1 < 10 => print!("{}{} ", " ".repeat(self.left_padding - 2), n + 1),
            n if n + 1 < 100 => print!("{}{} ", " ".repeat(self.left_padding - 3), n + 1),
            _ => panic!("Errors when printing Y-axis number."),
        }
    }
}
