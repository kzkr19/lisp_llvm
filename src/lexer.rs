use crate::types::{LispErr, Token, TokenKind};

pub struct Lexer {
    source: String,
    cursor: usize,
    tokens: Vec<Token>,
    token_index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let mut lexer = Lexer {
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
        Err(LispErr::Lexer("not implemented".to_string()))
    }
}
