use crate::types::{Expression, LispErr, SRange, Token, TokenKind};

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
                s => Err(LispErr::Parser(format!("found unknown symbol {}", s))),
            },
            _ => {
                self.consume_one_token()?;
                Ok(Some(Expression::Value(token.kind, token.range)))
            }
        }
    }

    fn parse_list(&mut self) -> Result<Option<Expression>, LispErr> {
        let start = self.peek_token(0).unwrap().range.start;
        self.consume_one_token()?; // consume (
        let mut v = vec![];

        loop {
            let token = match self.peek_token(0) {
                Some(v) => v,
                None => {
                    return Err(LispErr::Parser(format!(
                        "reached end of tokens when parsing list"
                    )))
                }
            };
            match token.kind {
                TokenKind::Symbol(symbol) => match symbol.as_str() {
                    ")" => break,
                    s if is_not_supported_symbol(s) => {
                        return Err(LispErr::NotSupported(format!(
                            "the symbol {} is not supported",
                            s
                        )))
                    }
                    _ => v.push(self.parse_expression()?.unwrap()),
                },
                _ => v.push(self.parse_expression()?.unwrap()),
            }
        }
        let end = self.peek_token(0).unwrap().range.end;
        self.consume_one_token()?; // consume )

        Ok(Some(self.expand_derived_expression(
            v,
            SRange {
                start: start,
                end: end,
            },
        )?))
    }

    fn expand_derived_expression(
        &self,
        xs: Vec<Expression>,
        range: SRange,
    ) -> Result<Expression, LispErr> {
        let mut xs = xs;
        if xs.len() == 0 {
            return Ok(Expression::List(xs, range));
        }

        match &xs[0] {
            Expression::List(_, _) => Ok(Expression::List(xs, range)),
            Expression::Value(x, _) => match x {
                TokenKind::Identifier(ident) => match ident.as_str() {
                    "and" => Err(LispErr::NotImplemented),
                    "or" => {
                        xs.remove(0);
                        Ok(self.expand_or(xs, range)?)
                    }
                    "begin" => self.expand_begin(xs, range),
                    "cond" => Err(LispErr::NotImplemented),
                    "case" => Err(LispErr::NotImplemented),
                    "let" => Err(LispErr::NotImplemented),
                    "let*" => Err(LispErr::NotImplemented),
                    "letrec" => Err(LispErr::NotImplemented),
                    "delay" => Err(LispErr::NotImplemented),
                    _ => Ok(Expression::List(xs, range)),
                },
                _ => Ok(Expression::List(xs, range)),
            },
        }
    }

    fn expand_begin(&self, mut xs: Vec<Expression>, range: SRange) -> Result<Expression, LispErr> {
        // convert (begin expr0 expr1) -> ((lambda (x0 x1) x1) expr0 expr1)
        xs.remove(0);

        if xs.len() == 0 {
            return Ok(Expression::Value(TokenKind::Boolean(false), range));
        }

        // create list for argument of lambda (like (x0 x1))
        let args_list = Expression::List(
            (0..xs.len())
                .map(|i| Expression::Value(TokenKind::Identifier(format!("__begin_x{}", i)), range))
                .collect::<Vec<Expression>>(),
            range,
        );
        // create definition of lambda(like (lambda (x0 x1) x1))
        let def_of_lambda = Expression::List(
            vec![
                Expression::Value(TokenKind::Identifier("lambda".to_string()), range),
                args_list,
                // returns last one
                Expression::Value(format!("__begin_x{}", xs.len() - 1), range),
            ],
            range,
        );

        // create list to call above lambda expression
        let call_lambda = Expression::List(
            {
                let mut temp = vec![def_of_lambda];
                temp.extend(xs);
                temp
            },
            range,
        );

        Ok(call_lambda)
    }

    fn expand_or(&self, mut xs: Vec<Expression>, range: SRange) -> Result<Expression, LispErr> {
        if xs.len() == 0 {
            return Ok(Expression::Value(TokenKind::Boolean(true), range));
        }

        let first = xs.pop().unwrap();

        Ok(Expression::List(
            vec![
                Expression::Value(TokenKind::Identifier("if".to_string()), xs[0].get_range()),
                first,
                Expression::Value(TokenKind::Boolean(true), range),
                if xs.len() == 0 {
                    Expression::Value(TokenKind::Boolean(false), range)
                } else {
                    self.expand_or(xs, range)?
                },
            ],
            range,
        ))
    }

    fn consume_one_token(&mut self) -> Result<(), LispErr> {
        if self.index < self.tokens.len() {
            self.index += 1;
            Ok(())
        } else {
            Err(LispErr::Parser(
                "Cannot consume token. We already reached the end of token".to_string(),
            ))
        }
    }

    fn peek_token(&self, n: usize) -> Option<Token> {
        match self.tokens.get(self.index + n) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}
