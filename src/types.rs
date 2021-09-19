#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Identifier(String), // def!,inc,dec,+,-,...
    Boolean(bool),
    Integer(i64),
    Character(char),
    Str(String),
    Symbol(String), // [],(),{},`,',@,~,~@,^
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SRange {
    pub start: usize,
    pub end: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: SRange,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    Value(TokenKind, SRange),
    List(Vec<Expression>, SRange),
}

#[derive(Debug)]
pub enum LispErr {
    Command(String),
    IO(String),
    Lexer(String),
    Parser(String),
    NotSupported(String),
    NotImplemented,
}

impl Expression {
    pub fn get_range(&self) -> SRange {
        match self {
            Expression::Value(_, x) => *x,
            Expression::List(_, x) => *x,
        }
    }
}
