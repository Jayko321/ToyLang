use crate::discord_script::{interpreter::Interpreter, parser, tokenizer};

pub mod discord_script;

fn main() {
    let code = "let a = 5; let b = 5 + a;";
    let tokens = tokenizer::Lexer::tokenize(code.to_string());
    let statement = parser::Parser::parse(tokens.unwrap()).unwrap();
    let mut inter = Interpreter::new();
    inter.null_expression_out = Some(Box::new(|val| {
        println!("{:?}", val);
    }));
    inter.on_error = Some(Box::new(|err| {
        println!("Error: {}", err);
    }));
    inter.execute(statement);
    println!("Hello, world!");
}
