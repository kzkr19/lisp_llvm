use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
pub mod lexer;
pub mod parser;
pub mod types;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::types::LispErr;

fn main() {
    match compile() {
        Ok(_) => {}
        Err(LispErr::Command(s)) => println!("command error:{}", s),
        Err(LispErr::IO(s)) => println!("io error:{}", s),
        Err(LispErr::Lexer(s)) => println!("lexer error:{}", s),
        Err(LispErr::Parser(s)) => println!("parser error:{}", s),
        Err(LispErr::NotSupported(s)) => println!("not supported error: {}", s),
        Err(LispErr::NotImplemented) => println!("not implemented!"),
    }
}

fn compile() -> Result<(), LispErr> {
    let fname = get_file_name()?;
    let source = read_code(fname)?;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.read_all_tokens()?;
    let mut parser = Parser::new(tokens);
    let _ast = parser.parse()?;

    Ok(())
}

fn get_file_name() -> Result<String, LispErr> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        Ok(args[1].clone())
    } else {
        Err(LispErr::Command(
            "invalid command line argument".to_string(),
        ))
    }
}

fn read_code(fname: String) -> Result<String, LispErr> {
    let file = match File::open(fname.clone()) {
        Ok(v) => v,
        Err(_) => return Err(LispErr::IO(format!("Cannot open file {}.", fname))),
    };
    let mut buf_reader = BufReader::new(file);
    let mut code = String::new();
    match buf_reader.read_to_string(&mut code) {
        Ok(_) => Ok(code),
        Err(_) => Err(LispErr::IO(format!("Cannot read file {}.", fname))),
    }
}
