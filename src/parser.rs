use crate::types::{LispData, LispErr, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let parser = Parser {
            tokens: tokens,
            index: 0,
        };

        parser
    }

    pub fn parse(&mut self) -> Result<Vec<LispData>, LispErr> {
        Err(LispErr::NotImplemented)
    }
}
