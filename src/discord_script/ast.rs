use super::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Types {
    Integer,
    UnsingedInteger,
    Float,
    String,
    Boolean,
    Pointer,
    Custom(String, usize),
    Void, //Identifier, size
}

impl Types {
    pub fn supports(&self, op: BinaryOperations) -> bool {
        match (self, op) {
            (Types::Integer, BinaryOperations::Add)
            | (Types::Integer, BinaryOperations::Subtract)
            | (Types::Integer, BinaryOperations::Divide)
            | (Types::Integer, BinaryOperations::Multiply)
            | (Types::UnsingedInteger, BinaryOperations::Add)
            | (Types::UnsingedInteger, BinaryOperations::Subtract)
            | (Types::UnsingedInteger, BinaryOperations::Divide)
            | (Types::UnsingedInteger, BinaryOperations::Multiply)
            | (Types::Float, BinaryOperations::Add)
            | (Types::Float, BinaryOperations::Subtract)
            | (Types::Float, BinaryOperations::Divide)
            | (Types::Float, BinaryOperations::Multiply)
            | (Types::String, BinaryOperations::Add) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct AbstractValue {
    pub memory: Box<Vec<u8>>,
    pub _type: Types,
}

impl AbstractValue {
    pub fn new(memory: Box<Vec<u8>>, _type: Types) -> Self {
        Self { memory, _type }
    }
}

impl From<AbstractValue> for i64 {
    fn from(value: AbstractValue) -> i64 {
        i64::from_be_bytes(value.memory.as_slice().try_into().unwrap())
    }
}

impl From<i64> for AbstractValue {
    fn from(value: i64) -> Self {
        Self::new(Box::new(value.to_be_bytes().to_vec()), Types::Integer)
    }
}

impl From<AbstractValue> for f64 {
    fn from(value: AbstractValue) -> f64 {
        f64::from_be_bytes(value.memory.as_slice().try_into().unwrap())
    }
}

impl From<f64> for AbstractValue {
    fn from(value: f64) -> Self {
        Self::new(Box::new(value.to_be_bytes().to_vec()), Types::Integer)
    }
}

impl From<AbstractValue> for u64 {
    fn from(value: AbstractValue) -> u64 {
        u64::from_be_bytes(value.memory.as_slice().try_into().unwrap())
    }
}

impl From<u64> for AbstractValue {
    fn from(value: u64) -> Self {
        Self::new(Box::new(value.to_be_bytes().to_vec()), Types::Integer)
    }
}

impl From<AbstractValue> for String {
    fn from(value: AbstractValue) -> Self {
        String::from_utf8(*value.memory).unwrap()
    }
}

impl From<&AbstractValue> for String {
    fn from(value: &AbstractValue) -> Self {
        let copied = *value.memory.clone();
        String::from_utf8(copied).unwrap()
    }
}

impl From<String> for AbstractValue {
    fn from(value: String) -> Self {
        Self::new(Box::new(value.into_bytes()), Types::String)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOperations {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl TryFrom<TokenKind> for BinaryOperations {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        use super::ast::BinaryOperations::*;
        use super::token::TokenKind::*;
        match value {
            Plus => Ok(Add),
            Minus => Ok(Subtract),
            TokenKind::Divide => Ok(BinaryOperations::Divide),
            Star => Ok(Multiply),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum AbstractExpressionDescription {
    Integer(AbstractValue),
    UnsingedInteger(AbstractValue),
    Float(AbstractValue),
    LiteralString(AbstractValue),
    Symbol(String),
    Binary(
        Box<AbstractExpressionDescription>,
        Box<AbstractExpressionDescription>,
        BinaryOperations,
    ),
    FunctionCall(
        Box<AbstractExpressionDescription>,
        usize,
        Vec<Box<AbstractExpressionDescription>>,
    ), //Identifier(SymbolExpression), ArgumentCount, Arguments
       //Groupping(), // TODO: Resolve before execution, (just unwind this so that an ast is correct
       // before execution, like parantheses were there)
}

impl PartialEq for AbstractExpressionDescription {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub enum AbstractStatementDescription {
    Variable(String, usize, Box<AbstractExpressionDescription>),
    Block(Vec<Box<AbstractStatementDescription>>),
    Expression(Box<AbstractExpressionDescription>),
}

pub trait Expression: std::fmt::Debug {
    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        todo!()
    }
    fn get_description(&self) -> AbstractExpressionDescription;
}

pub struct MemoryAllocationInfo {
    pub count: usize, //How much to allocate in bytes
}

pub trait Statement: std::fmt::Debug {
    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        todo!()
    }
    fn get_description(&self) -> AbstractStatementDescription;
}

//EXPRESSIONS

//LITERAL
#[derive(Debug)]
pub struct FloatExpression {
    pub value: f64,
}

#[derive(Debug)]
pub struct IntegerExpression {
    pub value: i128,
}

impl Expression for FloatExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        return AbstractExpressionDescription::Float(AbstractValue::new(
            Box::new(self.value.to_be_bytes().to_vec()),
            Types::Float,
        ));
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo { count: 8 }
    }
}

impl Expression for IntegerExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        if let Ok(value) = i64::try_from(self.value) {
            return AbstractExpressionDescription::Integer(AbstractValue::new(
                Box::new(value.to_be_bytes().to_vec()),
                Types::Integer,
            ));
        }
        if let Ok(value) = u64::try_from(self.value) {
            return AbstractExpressionDescription::UnsingedInteger(AbstractValue::new(
                Box::new(value.to_be_bytes().to_vec()),
                Types::UnsingedInteger,
            ));
        }
        panic!("Should never happen! Integer expression overwflow");
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo { count: 8 }
    }
}

#[derive(Debug)]
pub struct StringExpression {
    pub value: String,
}
impl Expression for StringExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        return AbstractExpressionDescription::LiteralString(AbstractValue::new(
            Box::new(self.value.clone().into_bytes()),
            Types::String,
        ));
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo {
            count: self.value.len(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolExpression {
    pub value: String,
}
impl Expression for SymbolExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        AbstractExpressionDescription::Symbol(self.value.clone())
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo { count: 0 }
    }
}

//COMPLEX
#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Expression>,
    pub operator: Token,
    pub right: Box<dyn Expression>,
}
impl Expression for BinaryExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        let op = match TryInto::<BinaryOperations>::try_into(self.operator.kind.clone()) {
            Ok(op) => op,
            Err(..) => panic!(
                "Should never happen! Binary expression had a wrong token {}",
                self.operator
            ),
        };

        AbstractExpressionDescription::Binary(
            Box::new(self.left.get_description()),
            Box::new(self.right.get_description()),
            op,
        )
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        MemoryAllocationInfo {
            count: self.left.get_memory_allocation_info().count
                + self.right.get_memory_allocation_info().count,
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub expression: Box<dyn Expression>,
}
impl Expression for UnaryExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        todo!()
    }
}

#[derive(Debug)]
pub struct GrouppingExpression {
    pub inner: Box<dyn Expression>,
}
impl Expression for GrouppingExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        todo!()
    }
}

#[derive(Debug)]
pub struct AssignmentExpression {
    pub assigne: Box<dyn Expression>,
    pub operator: Token,
    pub value: Box<dyn Expression>,
}
impl Expression for AssignmentExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        todo!()
    }
}

#[derive(Debug)]
pub struct FunctionCallExpression {
    pub params: Vec<Box<dyn Expression>>,
    pub identifier: Box<dyn Expression>,
}
impl Expression for FunctionCallExpression {
    fn get_description(&self) -> AbstractExpressionDescription {
        let params: Vec<Box<AbstractExpressionDescription>> = self
            .params
            .iter()
            .map(|v| Box::new(v.get_description()))
            .collect();
        AbstractExpressionDescription::FunctionCall {
            0: Box::new(self.identifier.get_description()),
            1: params.len(),
            2: params,
        }
    }
}

//STATEMENTS
#[derive(Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<dyn Statement>>,
}
impl Statement for BlockStatement {
    fn get_description(&self) -> AbstractStatementDescription {
        AbstractStatementDescription::Block {
            0: self
                .body
                .iter()
                .map(|v| Box::new(v.get_description()))
                .collect(),
        }
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        let mut count = 0;
        for statement in &self.body {
            count += statement.get_memory_allocation_info().count;
        }
        MemoryAllocationInfo { count }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}
impl Statement for ExpressionStatement {
    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        self.expression.get_memory_allocation_info()
    }

    fn get_description(&self) -> AbstractStatementDescription {
        AbstractStatementDescription::Expression {
            0: Box::new(self.expression.get_description()),
        }
    }
}

#[derive(Debug)]
pub struct VariableStatement {
    pub is_const: bool,
    pub variable_name: String,
    pub explicit_type: Option<String>,
    pub assignment: Box<dyn Expression>,
}
impl Statement for VariableStatement {
    fn get_description(&self) -> AbstractStatementDescription {
        AbstractStatementDescription::Variable(
            self.variable_name.clone(),
            self.assignment.get_memory_allocation_info().count,
            Box::new(self.assignment.get_description()),
        )
    }

    fn get_memory_allocation_info(&self) -> MemoryAllocationInfo {
        todo!()
    }
}
