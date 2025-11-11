use crate::event_script::{parser::Parser, tokenizer::Lexer, type_system::TypeChecker};

pub mod event_script;

fn main() {
    let code = "let a = 5; { let b = 5 + a; } let c = a + a; let d = 7; ";
    let tokens = Lexer::tokenize(code.to_string()).unwrap();
    // println!("{:?}", tokens);
    let stmt = Parser::parse(tokens);
    // println!("{:?}", stmt);
    let res = TypeChecker::check(stmt.unwrap());
    println!("{:?}", res);
}
