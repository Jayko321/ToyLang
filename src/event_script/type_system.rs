use std::{collections::HashMap, error::Error};

use crate::event_script::ast::Statement;

#[derive(Debug, PartialEq, Eq)]
pub struct Type {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    name: String,
    return_type: Type,
    params: Vec<Type>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Variable {
    name: String,
    is_const: bool,
    mutable: bool,
    type_: Option<Type>,
}

impl Variable {
    fn new<S: ToString>(name: S, is_const: bool, mutable: bool, type_: Option<Type>) -> Self {
        Self {
            name: name.to_string(),
            is_const,
            mutable,
            type_,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum SymbolType {
    Type(Type),
    Variable(Variable),
    Function(Function),
    // Macro(String, Vec<T>),
}

pub struct TypeChecker {
    symbol_table: HashMap<String, SymbolType>,
    // symbol_table:
}
#[derive(Debug)]
pub enum TypeErrors {
    TypeAlreadyExists(Type),
    FunctionAlreadyExists(Function),
    VariableAlreadyExists(Variable),
}

impl Type {
    pub fn new<S: ToString>(name: S, size: usize) -> Self {
        Type {
            name: name.to_string(),
            size,
        }
    }
}

impl TypeChecker {
    pub fn new() -> Self {
        let mut res = TypeChecker {
            symbol_table: HashMap::new(),
        };
        res.initialize();

        res
    }

    pub fn check(statements: Vec<Statement>) -> Result<(), TypeErrors> {
        let mut checker = TypeChecker::new();
        for stmt in statements {
            checker.populate_table(stmt)?;
        }
        Ok(())
    }

    fn populate_table(&mut self, statement: Statement) -> Result<(), TypeErrors> {
        todo!()
    }
    fn check_type_correctness(&mut self) {}

    pub(super) fn initialize(&mut self) {
        self.add_type("i8", 8).unwrap();
        self.add_type("i16", 16).unwrap();
        self.add_type("i32", 32).unwrap();
        self.add_type("i64", 64).unwrap();

        self.add_type("u8", 8).unwrap();
        self.add_type("u16", 16).unwrap();
        self.add_type("u32", 32).unwrap();
        self.add_type("u64", 64).unwrap();

        self.add_type("f32", 32).unwrap();
        self.add_type("f64", 64).unwrap();
    }

    pub fn add_type<S: ToString>(&mut self, name: S, size: usize) -> Result<(), String> {
        if self.symbol_table.contains_key(&name.to_string()) {
            return Err("Type already exists".to_string());
        }
        self.symbol_table
            .insert(name.to_string(), SymbolType::Type(Type::new(name, size)));
        Ok(())
    }

    pub fn add_variable<S: ToString>(
        &mut self,
        name: S,
        is_const: bool,
        mutable: bool,
        type_: Option<Type>,
    ) -> Result<(), String> {
        if self.symbol_table.contains_key(&name.to_string()) {
            return Err("Variable already exists".to_string());
        }
        self.symbol_table.insert(
            name.to_string(),
            SymbolType::Variable(Variable::new(name, is_const, mutable, type_)),
        );
        Ok(())
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
