use crate::types::{Expression, LispErr, Token, TokenKind};

fn is_not_supported_symbol(symbol: &str) -> bool {
    let vec = vec!["#(", "'", "`", "'", "`", ",", ",@", "."];

    for v in vec {
        if symbol == v {
            return true;
        }
    }

    false
}

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

    pub fn parse(&mut self) -> Result<Vec<Expression>, LispErr> {
        let mut v = vec![];
        while let Some(expr) = self.parse_expression()? {
            v.push(expr);
        }
        Ok(v)
    }

    fn parse_expression(&mut self) -> Result<Option<Expression>, LispErr> {
        let token = match self.peek_token(0) {
            Some(v) => v,
            None => return Ok(None),
        };

        match token.kind {
            TokenKind::Symbol(symbol) => match symbol.as_str() {
                "(" => self.parse_list(),
                ")" => Err(LispErr::Parser("unexpected ')'".to_string())),
                s if is_not_supported_symbol(s) => Err(LispErr::NotSupported(format!(
                    "the symbol {} is not supported",
                    s
                ))),
                s => Err(LispErr::NotImplemented),
            },
            _ => Ok(Some(Expression::Value(token))),
        }
    }

    fn parse_list(&mut self) -> Result<Option<Expression>, LispErr> {
        Err(LispErr::NotImplemented)
    }

    fn peek_token(&self, n: usize) -> Option<Token> {
        match self.tokens.get(self.index + n) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}
