use crate::types::{LispErr, Token, TokenKind};
use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

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
    regex_set: HashMap<String, Regex>,
}

fn create_regex() -> HashMap<String, Regex> {
    let mut hm = HashMap::new();

    hm.insert(
        format!("boolean"),
        Regex::new(format!(r"((#f)|(#t))").as_str()).unwrap(),
    );
    hm.insert(
        format!("character"),
        Regex::new(format!(r"(#\\)(.|(space)|(newline))").as_str()).unwrap(),
    );

    let letter = format!(r"[A-Za-z]");
    let special_initial = format!(r"(!|\$|%|&|\*|/|:|<|=|>|\?|\^|_|~)");
    let initial = format!("({}|{})", letter, special_initial);
    let digit = format!(r"\d");
    let special_subsequent = format!(r"(\+|-|\.|@)");
    let subsequent = format!("({}|{}|{})", initial, digit, special_subsequent);

    let peculiar_identifier = format!(r"(\+|-|>=|<=|<|>)");
    hm.insert(
        format!("identifier"),
        Regex::new(format!("(({}({})*)|{})", initial, subsequent, peculiar_identifier).as_str())
            .unwrap(),
    );

    hm.insert(
        format!("symbol"),
        Regex::new(format!(r"(\(|\)|(#\()|'|`|,@|,|\.)").as_str()).unwrap(),
    );

    hm.insert(
        format!("string"),
        Regex::new(format!(r#"([^"\\]|(\\")|(\\\\))*""#).as_str()).unwrap(),
    );

    let sign = format!(r"(\+|-)?");
    for (radix, radix_hash, digit) in vec![
        (2, "#b", "(0|1)"),
        (8, "#o", "[0-7]"),
        (10, "(#d)?", r"\d"),
        (16, "#x", r"(\d|[a-fA-F])"),
    ] {
        hm.insert(
            format!("integer_{}", radix),
            Regex::new(format!(r"{}{}({})+", radix_hash, sign, digit).as_str()).unwrap(),
        );
    }

    hm
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let lexer = Lexer {
            source: source,
            cursor: 0,
            tokens: vec![],
            token_index: 0,
            regex_set: create_regex(),
        };

        lexer
    }
}

impl Lexer {
    pub fn read_all_tokens(&mut self) -> Result<(), LispErr> {
        loop {
            self.skip_atmosphere();
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
        println!("{:?}", self.tokens);

        Ok(())
    }

    fn read_next_token(&mut self) -> Result<Option<TokenKind>, LispErr> {
        if self.is_eof() {
            return Ok(None);
        }

        continue_if_none!(self.read_identifier());
        continue_if_none!(self.read_symbol());
        continue_if_none!(self.read_character());
        continue_if_none!(self.read_boolean());
        continue_if_none!(self.read_number());

        Err(LispErr::Lexer(format!(
            "Couldn't read token. cursor={}",
            self.cursor
        )))
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
        if let Some(v) = self.read_token(&String::from("identifier")) {
            Ok(Some(TokenKind::Identifier(v)))
        } else {
            Ok(None)
        }
    }

    fn read_symbol(&mut self) -> Result<Option<TokenKind>, LispErr> {
        if let Some(v) = self.read_token(&String::from("symbol")) {
            Ok(Some(TokenKind::Symbol(v)))
        } else {
            Ok(None)
        }
    }

    fn read_character(&mut self) -> Result<Option<TokenKind>, LispErr> {
        if let Some(v) = self.read_token(&String::from("character")) {
            let c = match v.as_str() {
                "#\\space" => ' ',
                "#\\newline" => '\n',
                v => v.chars().nth(2).unwrap(),
            };
            Ok(Some(TokenKind::Character(c)))
        } else {
            Ok(None)
        }
    }

    fn read_boolean(&mut self) -> Result<Option<TokenKind>, LispErr> {
        if let Some(v) = self.read_token(&String::from("boolean")) {
            match v.as_str() {
                "#t" => Ok(Some(TokenKind::Boolean(true))),
                "#f" => Ok(Some(TokenKind::Boolean(false))),
                _ => Err(LispErr::Lexer(format!(
                    "Unkown string received when reading boolean: {}",
                    v
                ))),
            }
        } else {
            Ok(None)
        }
    }

    fn read_number(&mut self) -> Result<Option<TokenKind>, LispErr> {
        // TODO: remove magic numbers
        for radix in vec![2, 8, 10, 16] {
            let key = format!("integer_{}", radix);
            if let Some(v) = self.read_token(&key) {
                let num = if v.starts_with("#") {
                    v[2..].to_string()
                } else {
                    v
                };

                return match i64::from_str_radix(num.as_str(), radix) {
                    Ok(val) => Ok(Some(TokenKind::Integer(val))),
                    Err(_) => Err(LispErr::Lexer(format!(
                        "Failed to parse \"{}\" in radix {}",
                        num, radix
                    ))),
                };
            }
        }
        Ok(None)
    }

    fn read_token(&mut self, key: &String) -> Option<String> {
        let s = &self.source[self.cursor..];
        let result = self.regex_set[key].find(s);

        if let Some(m) = result {
            if m.start() == 0 {
                let len = m.end() - m.start();
                let token = String::from(&self.source[self.cursor..self.cursor + len]);
                self.move_cursor(len as i64);
                Some(token)
            } else {
                None
            }
        } else {
            None
        }
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

    fn move_cursor(&mut self, n_step: i64) {
        self.cursor = (self.cursor as i64 + n_step) as usize;
        self.cursor = min(self.cursor, self.source.len());
    }
}
