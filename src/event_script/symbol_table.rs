use std::collections::HashMap;

use crate::event_script::type_system::{Type, Variable};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SymbolType {
    Type(Type),
    Variable(Variable),
    // Function(Function),
    // Macro(String, Vec<T>),
}

impl SymbolType {
    pub fn get_symbol_typename(&self) -> String {
        match self {
            SymbolType::Type(_) => String::from("Type"),
            SymbolType::Variable(_) => String::from("Var"),
        }
    }

    pub fn get_symbol_name(&self) -> String {
        match self {
            SymbolType::Type(ty) => ty.name.clone(),
            SymbolType::Variable(variable) => variable.name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    storage: Vec<SymbolType>,
    storage_view: HashMap<String, usize>,
    // current_depth: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            storage_view: HashMap::new(),
        }
    }

    pub(super) fn generate_symbol_name(
        name: &String,
        symbol: &SymbolType,
        scope_name: &str,
        depth: u16,
    ) -> String {
        name.to_owned()
            + "'"
            + [
                symbol.get_symbol_typename().as_str(),
                depth.to_string().as_str(),
                scope_name,
            ]
            .join("'")
            .as_str()
    }

    pub fn insert(&mut self, symbol: SymbolType, depth: u16, scope_name: &str) {
        let name =
            Self::generate_symbol_name(&symbol.get_symbol_name(), &symbol, scope_name, depth);
        if self.storage_view.contains_key(&name) {
            return;
        }
        self.storage.push(symbol);
        self.storage_view.insert(name, self.storage.len() - 1);
    }

    pub fn get_type(&self, name: &String, depth: u16, scope_name: &str) -> Option<&Type> {
        let name =
            Self::generate_symbol_name(name, &SymbolType::Type(Type::default()), scope_name, depth);
        let idx = self.storage_view.get(&name)?;
        let symbol = self.storage.get(*idx)?;
        match symbol {
            SymbolType::Type(ty) => Some(ty),
            SymbolType::Variable(_) => None,
        }
    }

    pub fn get_variable(&self, name: &String, depth: u16, scope_name: &str) -> Option<&Variable> {
        let name = Self::generate_symbol_name(
            name,
            &SymbolType::Variable(Variable::default()),
            scope_name,
            depth,
        );
        let idx = self.storage_view.get(&name)?;
        let symbol = self.storage.get(*idx)?;
        match symbol {
            SymbolType::Type(_) => None,
            SymbolType::Variable(var) => Some(var),
        }
    }

    pub fn is_name_taken(&self, _name: &String) -> bool {
        false
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
