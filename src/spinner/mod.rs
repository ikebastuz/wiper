#[derive(Debug)]
pub struct Spinner {
    symbols: Vec<char>,
    done: char,
    current: usize,
}

impl Spinner {
    fn new() -> Self {
        Spinner {
            symbols: vec!['⠁', '⠂', '⠄', '⡀', '⢀', '⠠', '⠐', '⠈'],
            done: '⣿',
            current: 0,
        }
    }

    fn move_position(&mut self, steps: isize) {
        let len = self.symbols.len() as isize;
        self.current = (((self.current as isize + steps) % len + len) % len) as usize;
    }

    pub fn get_icons(&mut self, is_loaded: bool) -> (char, char) {
        if is_loaded {
            return (self.done, self.done);
        }
        self.move_position(1);

        let right_symbol = self.symbols[self.current];
        let left_symbol_index = (self.current + self.symbols.len() / 2) % self.symbols.len();
        let left_symbol = self.symbols[left_symbol_index];
        (left_symbol, right_symbol)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
