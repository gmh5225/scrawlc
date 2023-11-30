use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(index: usize, line: usize, column: usize) -> Self {
        Position {
            index,
            line,
            column,
        }
    }

    pub fn default() -> Self {
        Position::new(0, 0, 0)
    }

    pub fn advance(&mut self, char: char) {
        self.index += 1;

        if char == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
