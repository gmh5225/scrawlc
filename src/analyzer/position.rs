use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    /// Returns a new position structure using the given arguments.
    ///
    /// # Examples
    /// ```
    /// let pos = scrawlc::Position::new(1, 2, 3);
    ///
    /// assert_eq!(pos.index, 1);
    /// assert_eq!(pos.line, 2);
    /// assert_eq!(pos.column, 3);
    /// ```
    pub fn new(index: usize, line: usize, column: usize) -> Self {
        Position {
            index,
            line,
            column,
        }
    }

    /// Returns a new position structure with all values being zero.
    ///
    /// # Examples
    /// ```
    /// let pos = scrawlc::Position::default();
    ///
    /// assert_eq!(pos.index, 0);
    /// assert_eq!(pos.line, 0);
    /// assert_eq!(pos.column, 0);
    /// ```
    pub fn default() -> Self {
        Position::new(0, 0, 0)
    }

    /// Advances the position's values upon the `char` parameter.
    /// If it is a new line, it will update line and reset column;
    /// if it is another character, it will keep line as same and update column.
    ///
    /// # Examples
    /// ```
    /// let mut pos = scrawlc::Position::default();
    ///
    /// pos.advance('c');
    /// pos.advance('h');
    ///
    /// assert_eq!(pos.index, 2);
    /// assert_eq!(pos.column, 2);
    ///
    /// pos.advance('\n');
    ///
    /// assert_eq!(pos.index, 3);
    /// assert_eq!(pos.line, 1);
    /// assert_eq!(pos.column, 0);
    /// ```
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
    /// Formats a string by updating line and column.
    ///
    /// # Examples
    /// ```
    /// let mut pos = scrawlc::Position::default();
    ///
    /// assert_eq!(pos.to_string(), "1:1");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
