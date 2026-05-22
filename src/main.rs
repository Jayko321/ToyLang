use clap::Parser;

use crate::event_script::{
    parser::Parser as LangParser,
    tokenizer::Lexer,
    type_system::TypeChecker,
};

pub mod event_script;

#[derive(Parser)]
#[command(name = "event-script", version, about = "A toy programming language")]
struct Cli {
    /// Source file to process (use "-" for stdin)
    file: Option<String>,

    /// Print the token stream
    #[arg(short, long)]
    tokens: bool,

    /// Print the parsed AST
    #[arg(short, long)]
    ast: bool,

    /// Run the type checker
    #[arg(short, long)]
    check: bool,
}

fn main() {
    let cli = Cli::parse();

    let source = match &cli.file {
        None => {
            Cli::parse_from(&["event-script", "--help"]);
            unreachable!();
        }
        Some(file) if file == "-" => read_stdin(),
        Some(file) => {
            std::fs::read_to_string(file).unwrap_or_else(|e| {
                eprintln!("Error reading file '{file}': {e}");
                std::process::exit(1);
            })
        }
    };

    let tokens = Lexer::tokenize(source).unwrap_or_else(|e| {
        eprintln!("Lexer error: {e}");
        std::process::exit(1);
    });

    if cli.tokens {
        println!("Tokens: {tokens:#?}");
    }

    let stmts = LangParser::parse(tokens).unwrap_or_else(|e| {
        eprintln!("Parser error: {e:?}");
        std::process::exit(1);
    });

    if cli.ast {
        println!("AST: {stmts:#?}");
    }

    if cli.check {
        TypeChecker::check(stmts).unwrap_or_else(|e| {
            eprintln!("Type error: {e:?}");
            std::process::exit(1);
        });
        println!("Type check passed.");
    }
}

fn read_stdin() -> String {
    use std::io::Read;
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
        eprintln!("Error reading stdin: {e}");
        std::process::exit(1);
    });
    buf
}
