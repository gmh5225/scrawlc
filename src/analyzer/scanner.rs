mod position;
mod token;

pub use position::Position;
pub use token::Token;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("cannot access {0}, end of content")]
    EndOfContent(Position),
}

#[derive(Debug)]
pub struct Scanner {
    cont: String,
    cur_pos: Position,
    cur_char: char,
}

impl Scanner {
    /// Returns a new scanner structure with the given values.
    ///
    /// # Errors
    /// `ScannerError::EndOfContent`: If the given position's index is higher than content's length.
    ///
    /// # Examples
    /// ```
    /// let scanner = scrawlc::Scanner::with_position("example content", &scrawlc::Position::default()).unwrap();
    ///
    /// assert_eq!(scanner.content(), "example content");
    /// assert_eq!(scanner.current_position(), &scrawlc::Position::new(0, 0, 0));
    /// assert_eq!(scanner.current_character(), 'e');
    /// ```
    pub fn with_position(content: &str, position: &Position) -> Result<Self, ScannerError> {
        let mut s = Scanner {
            cont: content.to_string(),
            cur_pos: position.clone(),
            cur_char: ' ',
        };

        s.cur_char = match s.cont.chars().nth(s.cur_pos.index) {
            Some(character) => character,
            None => return Err(ScannerError::EndOfContent(s.cur_pos)),
        };

        Ok(s)
    }

    /// Returns a new scanner structure with empty values.
    ///
    /// # Errors
    /// `ScannerError::EndOfContent`: If the given content is empty.
    ///
    /// # Examples
    /// ```
    /// let scanner = scrawlc::Scanner::new("example content").unwrap();
    ///
    /// assert_eq!(scanner.content(), "example content");
    /// assert_eq!(scanner.current_position(), &scrawlc::Position::new(0, 0, 0));
    /// assert_eq!(scanner.current_character(), 'e');
    /// ```
    pub fn new(content: &str) -> Result<Self, ScannerError> {
        Scanner::with_position(content, &Position::default())
    }

    /// Returns the scanner content.
    ///
    /// # Examples
    /// ```
    /// let scanner = scrawlc::Scanner::new("example content").unwrap();
    ///
    /// assert_eq!(scanner.content(), "example content");
    /// ```
    pub fn content(&self) -> &String {
        &self.cont
    }

    /// Returns the current scanner position.
    ///
    /// # Examples
    /// ```
    /// let scanner = scrawlc::Scanner::new("example content").unwrap();
    ///
    /// assert_eq!(scanner.current_position(), &scrawlc::Position::new(0, 0, 0));
    /// ```
    pub fn current_position(&self) -> &Position {
        &self.cur_pos
    }

    /// Returns the current scanner position.
    ///
    /// # Examples
    /// ```
    /// let scanner = scrawlc::Scanner::new("example content").unwrap();
    ///
    /// assert_eq!(scanner.current_character(), 'e');
    /// ```
    pub fn current_character(&self) -> char {
        self.cur_char
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut _result: Vec<Token> = Vec::new();

        Ok(_result)
    }
}
