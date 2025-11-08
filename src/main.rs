use crate::event_script::{parser::Parser, tokenizer::Lexer, type_system::TypeChecker};

pub mod discord_script;
pub mod event_script;

fn main() {
    let code = "let a = 5; { let b = 5 + a; } let c = a + b; ";
    let tokens = Lexer::tokenize(code.to_string()).unwrap();
    // println!("{:?}", tokens);
    let stmt = Parser::parse(tokens);
    println!("{:?}", stmt);
    _ = TypeChecker::check(stmt.unwrap());

    // let statement = parser::Parser::parse(tokens.unwrap()).unwrap();
    // let mut inter = Interpreter::new();
    // inter.null_expression_out = Some(Box::new(|val| {
    //     println!("{:?}", val);
    // }));
    // inter.on_error = Some(Box::new(|err| {
    //     println!("Error: {}", err);
    // }));
    // inter.execute(statement);
    // println!("Hello, world!");
}
