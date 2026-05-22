# Event Script

A toy programming language implemented in Rust — a learning project exploring compiler frontend construction.

## Features

- **Regex-based tokenizer** — lexes keywords, operators, literals, identifiers, and comments
- **Pratt parser** (top-down operator precedence) — correctly handles `*`/`/` binding tighter than `+`/`-`, with parenthesis overrides
- **Type checker** — infers the smallest-fit integer type (`i8`–`i64`, `u8`–`u64`, `f32`/`f64`) and validates binary operations
- **Scope-aware symbol table** — variables live in block scopes with depth tracking

## Usage

```bash
# Process a file
cargo run -- assets/demo.es -t -a -c

# Read from stdin
echo "let x = 42;" | cargo run -- -c -
```

| Flag             | Description            |
| ---------------- | ---------------------- |
| `-t`, `--tokens` | Print the token stream |
| `-a`, `--ast`    | Print the parsed AST   |
| `-c`, `--check`  | Run the type checker   |
| `-h`, `--help`   | Print help             |

## Demo

Run the included demo:

```bash
cargo run -- assets/demo.es -t -a -c
```

This showcases:

- **Pratt parsing** — `1 + 2 * 3` parses as `1 + (2 * 3)`, `(1 + 2) * 3` as `(1 + 2) * 3`
- **Type resolution** — `42` → `i8`, `300` → `i16`, `100000` → `i32`, `3000000000` → `i64`
- **Block scoping** — variables declared inside `{ }` are locally scoped

## Project structure

```
src/
├── main.rs                    # CLI entrypoint
└── event_script/
    ├── mod.rs                 # Module declarations
    ├── token.rs               # Token types and keyword mapping
    ├── tokenizer.rs           # Lexer / tokenizer
    ├── ast.rs                 # AST node definitions
    ├── parser.rs              # Pratt parser orchestration
    ├── expression_parser.rs   # Expression parsing (binary, unary, grouping)
    ├── statement_parser.rs    # Statement parsing (variable decl, blocks)
    ├── type_system.rs         # Type checker and type definitions
    └── symbol_table.rs        # Scope-aware symbol table
```

## License

GNU General Public License v3.0
