#[derive(Clone)]
pub struct Player {
    symbol: Box<String>,
}

impl Player {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: Box::new(String::from(symbol)),
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }
}
