// #![warn(clippy::pedantic)]
// #![allow(clippy::match_wildcard_for_single_variants)]
// #![allow(clippy::match_same_arms)]
use crate::event_script::{
    parser::Parser,
    symbol_table::{SymbolTable, SymbolType},
    tokenizer::Lexer,
    type_system::{Type, TypeChecker},
};

pub mod event_script;

fn main() {
    let code = "let mut a = 5; { let b = 5 + a; } let c = a + a; let d = 7; ";
    let tokens = Lexer::tokenize(code.to_string()).unwrap();
    // println!("{:?}", tokens);
    let stmt = Parser::parse(tokens);
    // println!("{:?}", stmt);
    // let res = TypeChecker::check(stmt.unwrap());
    let mut table = SymbolTable::new();
    table.insert_symbol(SymbolType::Type(Type::new(&"a", 8)), 2);
    table.insert_symbol(SymbolType::Type(Type::new(&"a", 8)), 1);
    println!("{table:?}");
}
