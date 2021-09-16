use crate::types::{LispErr, Token, TokenKind};

macro_rules! continue_if_none {
    ($x:expr) => {
        match $x {
            Ok(Some(v)) => return Ok(Some(v)),
            Ok(None) => {}
            Err(v) => return Err(v),
        }
    };
}

pub struct Lexer {
    source: String,
    cursor: usize,
    tokens: Vec<Token>,
    token_index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let lexer = Lexer {
            source: source,
            cursor: 0,
            tokens: vec![],
            token_index: 0,
        };

        lexer
    }
}

impl Lexer {
    pub fn read_all_tokens(&mut self) -> Result<(), LispErr> {
        loop {
            let cursor = self.cursor;
            if let Some(token) = self.read_next_token()? {
                self.tokens.push(Token {
                    kind: token,
                    start: cursor,
                });
            } else {
                break;
            }
        }

        Ok(())
    }

    fn read_next_token(&mut self) -> Result<Option<TokenKind>, LispErr> {
        self.skip_atmosphere();
        if self.is_eof() {
            return Ok(None);
        }

        let current_cursor = self.cursor;

        continue_if_none!(self.read_identifier());

        Err(LispErr::NotImplemented)
    }

    fn consume_character(&mut self, expected_char: char) -> Result<(), LispErr> {
        let c = self.peek_char(0);
        if c.is_none() {
            Err(LispErr::Lexer(format!(
                "Expected {}, but we got EOF.",
                expected_char
            )))
        } else if c.unwrap() == expected_char {
            Ok(())
        } else {
            Err(LispErr::Lexer(format!(
                "Expected {}, but we got {}.",
                expected_char,
                c.unwrap()
            )))
        }
    }

    fn skip_atmosphere(&mut self) {
        while let Some(c) = self.peek_char(0) {
            match c {
                ' ' | '\t' | '\r' | '\n' => self.inc_cursor(),
                ';' => {
                    self.skip_to_next_line();
                    self.inc_cursor();
                }
                _ => break,
            }
        }
    }

    fn skip_to_next_line(&mut self) {
        while let Some(c) = self.peek_char(0) {
            self.inc_cursor();
            match c {
                '\n' => return,
                _ => {}
            }
        }
    }

    fn read_identifier(&mut self) -> Result<Option<TokenKind>, LispErr> {
        // let letter = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        // let special_initial = "!$%&*/:<=>?^_~";
        // let digit = "01234567889";
        // let c = self.peek_char(0).unwrap();

        // TODO: not implemented!
        Err(LispErr::NotImplemented)
    }

    fn peek_char(&self, nth: usize) -> Option<char> {
        self.source.chars().nth(self.cursor + nth)
    }

    fn is_eof(&self) -> bool {
        self.cursor == self.source.len()
    }

    fn inc_cursor(&mut self) {
        if !self.is_eof() {
            self.cursor += 1;
        }
    }
}
