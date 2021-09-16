#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Identifier(String), // def!,inc,dec,+,-,...
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Character(char),
    Str(String),
    Symbol(String), // [],(),{},`,',@,~,~@,^
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
}

#[derive(Debug)]
pub enum LispErr {
    Command(String),
    IO(String),
    Lexer(String),
    NotImplemented,
}
