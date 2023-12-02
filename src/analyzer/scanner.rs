mod position;
mod token;

pub use position::Position;
pub use token::Token;

use thiserror::Error;

pub const ETX: char = 0x03 as char; // End Of Text
pub const TAB: char = 0x09 as char; // Tab ('\t')
pub const LF: char = 0x0A as char; // Line Feed ('\n')
pub const CR: char = 0x0D as char; // Carriage Return ('\r')

pub const IDENTIFIER_SET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_";
pub const NUMBER_SET: &str = "0123456789";

#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("cannot access {0}, end of content")]
    EndOfContent(Position),

    #[error("{0} is an unsupported character")]
    UnsupportedCharacter(char),
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
    /// assert_eq!(scanner.content(), &format!("example content{}{}", scrawlc::LF, scrawlc::ETX));
    /// assert_eq!(scanner.current_position(), &scrawlc::Position::new(0, 0, 0));
    /// assert_eq!(scanner.current_character(), 'e');
    /// ```
    pub fn with_position(content: &str, position: &Position) -> Result<Self, ScannerError> {
        let mut s = Scanner {
            cont: format!("{}{}{}", content, LF, ETX),
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
    /// assert_eq!(scanner.content(), &format!("example content{}{}", scrawlc::LF, scrawlc::ETX));
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
    /// assert_eq!(scanner.content(), &format!("example content{}{}", scrawlc::LF, scrawlc::ETX));
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

    /// Peeks the next character.
    fn peek_next(&self) -> Result<char, ScannerError> {
        match self.cont.chars().nth(self.cur_pos.index + 1) {
            Some(new_character) => Ok(new_character),
            None => return Err(ScannerError::EndOfContent(self.cur_pos)),
        }
    }

    /// This function solves a bug.
    // ! Do not touch it, and do not try to solve the bug.
    // ! I fear it may cause bigger bugs.
    fn stay(&self) -> Result<Position, ScannerError> {
        Ok(self.cur_pos)
    }

    /// Advances the scanner by one.
    /// Returns the old position if succeeds.
    ///
    /// # Errors
    /// `ScannerError::EndOfContent`: If cannot get the next character.
    ///
    /// # Examples
    /// ```
    /// let mut scanner = scrawlc::Scanner::new("example content").unwrap();
    ///
    /// assert_eq!(scanner.current_position().index, 0);
    /// assert_eq!(scanner.current_character(), 'e');
    ///
    /// scanner.advance().unwrap();
    ///
    /// assert_eq!(scanner.current_position().index, 1);
    /// assert_eq!(scanner.current_character(), 'x');
    /// ```
    pub fn advance(&mut self) -> Result<Position, ScannerError> {
        let clone = self.cur_pos.clone();

        self.cur_pos.advance(self.cur_char);
        self.cur_char = match self.cont.chars().nth(self.cur_pos.index) {
            Some(new_character) => new_character,
            None => return Err(ScannerError::EndOfContent(self.cur_pos)),
        };

        Ok(clone)
    }

    /// Scans the content.
    /// See the scanner specification for more information.
    ///
    /// # Errors
    /// `ScannerError::EndOfContent`: If advancing fails.
    /// `ScannerError::UnsupportedCharacter`: If the character is unsupported/unknown.
    ///
    /// # Examples
    /// See the scanner documentation for detailed examples.
    pub fn scan(&mut self) -> Result<Vec<Token>, ScannerError> {
        let mut _result: Vec<Token> = Vec::new();

        while self.peek_next()? != ETX {
            if IDENTIFIER_SET.contains(self.cur_char) {
                let mut identifier = String::new();

                while IDENTIFIER_SET.contains(self.cur_char) {
                    identifier.push(self.cur_char);

                    self.advance()?;
                }

                _result.push(Token::new("identifier", &identifier, &self.cur_pos))
            } else if NUMBER_SET.contains(self.cur_char) {
                let mut number = String::new();

                while IDENTIFIER_SET.contains(self.cur_char) {
                    number.push(self.cur_char);

                    self.advance()?;
                }

                _result.push(Token::new("number", &number, &self.cur_pos))
            } else {
                match self.cur_char {
                    ETX => self.advance()?,
                    TAB => self.advance()?,
                    LF => self.advance()?,
                    CR => self.advance()?,
                    ' ' => self.advance()?,
                    '(' => {
                        _result.push(Token::new("(", "(", &self.cur_pos));

                        self.advance()?
                    }
                    ')' => {
                        _result.push(Token::new(")", ")", &self.cur_pos));

                        self.advance()?
                    }
                    '{' => {
                        _result.push(Token::new("{", "{", &self.cur_pos));

                        self.advance()?
                    }
                    '}' => {
                        _result.push(Token::new("}", "}", &self.cur_pos));

                        self.advance()?
                    }
                    '[' => {
                        _result.push(Token::new("[", "[", &self.cur_pos));

                        self.advance()?
                    }
                    ']' => {
                        _result.push(Token::new("]", "]", &self.cur_pos));

                        self.advance()?
                    }
                    ';' => {
                        _result.push(Token::new(";", ";", &self.cur_pos));

                        self.advance()?
                    }
                    ',' => {
                        _result.push(Token::new(",", ",", &self.cur_pos));

                        self.advance()?
                    }
                    '.' => {
                        _result.push(Token::new(".", ".", &self.cur_pos));

                        self.advance()?
                    }
                    '@' => {
                        _result.push(Token::new("@", "@", &self.cur_pos));

                        self.advance()?
                    }
                    '#' => {
                        _result.push(Token::new("#", "#", &self.cur_pos));

                        self.advance()?
                    }
                    '=' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("==", "==", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("=", "=", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '+' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("+=", "+=", &self.cur_pos));

                                self.advance()?
                            }
                            '-' => {
                                _result.push(Token::new("++", "++", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("+", "+", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '>' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new(">=", ">=", &self.cur_pos));

                                self.advance()?
                            }
                            '>' => {
                                self.advance()?;

                                match self.cur_char {
                                    '=' => {
                                        _result.push(Token::new(">>=", ">>=", &self.cur_pos));

                                        self.advance()?
                                    }
                                    _ => {
                                        _result.push(Token::new(">>", ">>", &self.cur_pos));

                                        self.stay()?
                                    }
                                }
                            }
                            _ => {
                                _result.push(Token::new(">", ">", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '-' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("-=", "-=", &self.cur_pos));

                                self.advance()?
                            }
                            '-' => {
                                _result.push(Token::new("--", "--", &self.cur_pos));

                                self.advance()?
                            }
                            '>' => {
                                _result.push(Token::new("->", "->", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("-", "-", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '<' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("<=", "<=", &self.cur_pos));

                                self.advance()?
                            }
                            '<' => {
                                self.advance()?;

                                match self.cur_char {
                                    '=' => {
                                        _result.push(Token::new("<<=", "<<=", &self.cur_pos));

                                        self.advance()?
                                    }
                                    _ => {
                                        _result.push(Token::new("<<", "<<", &self.cur_pos));

                                        self.stay()?
                                    }
                                }
                            }
                            _ => {
                                _result.push(Token::new("<", "<", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '*' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("*=", "*=", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("*", "*", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '!' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("!=", "!=", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("!", "!", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '/' => {
                        self.advance()?;

                        match self.cur_char {
                            '/' => {
                                self.advance()?;

                                while self.peek_next()? != LF || self.peek_next()? != ETX {
                                    self.advance()?;
                                }

                                self.stay()?
                            }
                            '=' => {
                                _result.push(Token::new("/=", "/=", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("/", "/", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '~' => {
                        _result.push(Token::new("~", "~", &self.cur_pos));

                        self.advance()?
                    }
                    '&' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("&=", "&=", &self.cur_pos));

                                self.advance()?
                            }
                            '&' => {
                                _result.push(Token::new("&&", "&&", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("&", "&", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '?' => {
                        _result.push(Token::new("?", "?", &self.cur_pos));

                        self.advance()?
                    }
                    '|' => {
                        self.advance()?;

                        match self.cur_char {
                            '=' => {
                                _result.push(Token::new("|=", "|=", &self.cur_pos));

                                self.advance()?
                            }
                            '|' => {
                                _result.push(Token::new("||", "||", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("|", "|", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    ':' => {
                        self.advance()?;

                        match self.cur_char {
                            ':' => {
                                _result.push(Token::new("::", "::", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new(":", ":", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '^' => {
                        self.advance()?;

                        match self.cur_char {
                            '^' => {
                                _result.push(Token::new("^=", "^=", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("^", "^", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '%' => {
                        self.advance()?;

                        match self.cur_char {
                            '%' => {
                                _result.push(Token::new("%=", "%=", &self.cur_pos));

                                self.advance()?
                            }
                            _ => {
                                _result.push(Token::new("%", "%", &self.cur_pos));

                                self.stay()?
                            }
                        }
                    }
                    '"' => {
                        self.advance()?;
                        let mut string = String::new();

                        while self.cur_char != '"' {
                            string.push(self.cur_char);

                            self.advance()?;
                        }

                        self.advance()?
                    }
                    _ => return Err(ScannerError::UnsupportedCharacter(self.cur_char)),
                };
            }
        }

        Ok(_result)
    }
}
