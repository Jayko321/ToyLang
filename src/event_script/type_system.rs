use std::collections::HashMap;

use crate::event_script::ast::{Expression, Statement};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Type {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Function {
    name: String,
    return_type: Type,
    params: Vec<Type>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
enum SymbolType {
    Type(Type),
    Variable(Variable),
    Function(Function),
    // Macro(String, Vec<T>),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Key {
    name: String,
    depth: u16,
}

impl Key {
    pub fn new<S: ToString>(name: S, depth: u16) -> Self {
        Self {
            name: name.to_string(),
            depth,
        }
    }
}

pub struct TypeChecker {
    symbol_table: HashMap<Key, SymbolType>,
    // symbol_table:
}
#[derive(Debug)]
pub enum TypeErrors {
    TypeAlreadyExists(),
    FunctionAlreadyExists(),
    VariableAlreadyExists(),
    TypeNotFound(String),
    SymbolIsNotAType(String, String),
}

impl Type {
    pub fn new<S: ToString>(name: S, size: usize) -> Self {
        Type {
            name: name.to_string(),
            size,
        }
    }

    fn clone(&self) -> Type {
        Type::new(self.name.clone(), self.size)
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
            checker.populate_table(stmt, 0)?;
        }
        println!(
            "{:?}",
            checker
                .symbol_table
                .iter()
                .filter(|(_, val)| { matches!(val, SymbolType::Variable(_)) })
                .collect::<HashMap<&Key, &SymbolType>>()
        );
        Ok(())
    }

    fn populate_table(&mut self, statement: Statement, depth: u16) -> Result<(), TypeErrors> {
        match statement {
            Statement::Block(statements) => {
                for statement in statements {
                    let _depth = depth + 1;
                    self.populate_table(statement, _depth + 1)?;
                }
                Ok(())
            }
            Statement::Variable(name, is_const, explicit_type, expression) => {
                let mut _type = None;
                if let Some(type_name) = explicit_type {
                    if let Some(symbol) = self.symbol_table.get(&Key::new(name.clone(), depth)) {
                        match symbol {
                            SymbolType::Type(known_type) => _type = Some((*known_type).clone()),
                            _ => {
                                return Err(TypeErrors::SymbolIsNotAType(
                                    type_name.clone(),
                                    name.clone(),
                                ));
                            }
                        }
                    } else {
                        return Err(TypeErrors::TypeNotFound(type_name));
                    }
                }
                if let Some(expr) = expression {
                    _type = Some(self.solve_expression_type(expr)?);
                }
                self.add_variable(name, is_const, true, _type)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
    fn check_type_correctness(&mut self) {}
    fn solve_expression_type(&mut self, expression: Expression) -> Result<Type, TypeErrors> {
        match expression {
            Expression::String(_) => todo!(),
            Expression::Number(value) => {
                if value.parse::<i8>().is_ok() {
                    return Ok(Type::new("i8", 8));
                }
                if value.parse::<i16>().is_ok() {
                    return Ok(Type::new("i16", 16));
                }
                if value.parse::<i32>().is_ok() {
                    return Ok(Type::new("i32", 32));
                }
                if value.parse::<i64>().is_ok() {
                    return Ok(Type::new("i64", 64));
                }
                Err(TypeErrors::TypeNotFound("i?".to_string())) // todo: make proper error
            }
            Expression::Float(value) => {
                if value.parse::<f32>().is_ok() {
                    return Ok(Type::new("f32", 32));
                }
                if value.parse::<f64>().is_ok() {
                    return Ok(Type::new("f64", 64));
                }
                Err(TypeErrors::TypeNotFound("f?".to_string())) // todo: make proper error
            }
            Expression::Symbol(name) => {
                let symbols: HashMap<String, SymbolType> = self
                    .symbol_table
                    .iter()
                    .filter_map(|(key, val)| {
                        if key.name == name {
                            return Some((key.name.clone(), (*val).clone()));
                        }
                        None
                    })
                    .collect();

                if let Some(symbol) = symbols.get(&name) {
                    match symbol {
                        SymbolType::Type(_) => todo!(),
                        SymbolType::Variable(variable) => match &variable.type_ {
                            Some(type_) => return Ok((*type_).clone()),
                            None => todo!(),
                        },
                        SymbolType::Function(function) => todo!(),
                    }
                }

                todo!()
            }
            Expression::Assignment(expression) => Ok(self.solve_expression_type(*expression)?),
            Expression::Groupping(expression) => todo!(),
            Expression::Unary(token, expression) => todo!(),
            Expression::Binary(lhs_expr, token_kind, rhs_expr) => {
                let lhs = self.solve_expression_type(*lhs_expr)?;
                let rhs = self.solve_expression_type(*rhs_expr)?;
                if lhs == rhs {
                    Ok(lhs)
                } else {
                    Err(TypeErrors::TypeNotFound("??".to_string())) //todo: add proper error
                }
            }
        }
    }

    pub(super) fn initialize(&mut self) {
        self.add_type("i8", 8, 0).unwrap();
        self.add_type("i16", 16, 0).unwrap();
        self.add_type("i32", 32, 0).unwrap();
        self.add_type("i64", 64, 0).unwrap();

        self.add_type("u8", 8, 0).unwrap();
        self.add_type("u16", 16, 0).unwrap();
        self.add_type("u32", 32, 0).unwrap();
        self.add_type("u64", 64, 0).unwrap();

        self.add_type("f32", 32, 0).unwrap();
        self.add_type("f64", 64, 0).unwrap();
    }

    pub fn add_type<S: ToString>(
        &mut self,
        name: S,
        size: usize,
        depth: u16,
    ) -> Result<(), TypeErrors> {
        if self
            .symbol_table
            .contains_key(&Key::new(name.to_string(), depth))
        {
            return Err(TypeErrors::TypeAlreadyExists());
        }
        self.symbol_table.insert(
            Key::new(name.to_string(), depth),
            SymbolType::Type(Type::new(name, size)),
        );
        Ok(())
    }

    pub fn add_variable<S: ToString>(
        &mut self,
        name: S,
        is_const: bool,
        mutable: bool,
        type_: Option<Type>,
    ) -> Result<(), TypeErrors> {
        if self
            .symbol_table
            .contains_key(&Key::new(name.to_string(), 0))
        {
            return Err(TypeErrors::VariableAlreadyExists());
        }
        self.symbol_table.insert(
            Key::new(name.to_string(), 0),
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
