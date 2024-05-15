#[derive(Debug)]
pub struct Spinner {
    symbols: Vec<char>,
    current: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            symbols: vec!['⠁', '⠂', '⠄', '⡀', '⢀', '⠠', '⠐', '⠈'],
            current: 0,
        }
    }

    pub fn next(&mut self) -> char {
        let symbol = self.symbols[self.current];
        self.current = (self.current + 1) % self.symbols.len();
        symbol
    }

    pub fn done(&self) -> char {
        '⣿'
    }
}
