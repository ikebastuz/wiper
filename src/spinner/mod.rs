#[derive(Debug)]
pub struct Spinner {
    symbols: Vec<char>,
    current: usize,
}

impl Spinner {
    fn new() -> Self {
        Spinner {
            symbols: vec!['⠁', '⠂', '⠄', '⡀', '⢀', '⠠', '⠐', '⠈'],
            current: 0,
        }
    }

    pub fn next_item(&mut self) -> char {
        let symbol = self.symbols[self.current];
        self.current = (self.current + 1) % self.symbols.len();
        symbol
    }

    pub fn done(&self) -> char {
        '⣿'
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
