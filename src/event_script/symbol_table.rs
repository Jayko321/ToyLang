use std::collections::HashMap;

use crate::event_script::type_system::{Type, Variable};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SymbolType {
    Type(Type),
    Variable(Variable),
    // Function(Function),
    // Macro(String, Vec<T>),
}

#[derive(Debug)]
pub struct SymbolTable {
    storage: Vec<SymbolType>,
    depth_table_view: HashMap<u16, HashMap<String, usize>>,
    type_table_view: HashMap<String, Vec<usize>>,
    variable_table_view: HashMap<String, Vec<usize>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            depth_table_view: HashMap::new(),
            type_table_view: HashMap::new(),
            variable_table_view: HashMap::new(),
        }
    }

    pub fn insert_symbol(&mut self, symbol: SymbolType, depth: u16) {
        let (inserted, name) = match &symbol {
            SymbolType::Type(type_) => (
                !self.type_table_view.contains_key(type_.name.as_str()),
                type_.name.as_str(),
            ),
            SymbolType::Variable(variable) => (
                !self
                    .variable_table_view
                    .contains_key(variable.name.as_str()),
                variable.name.as_str(),
            ),
        };
        let count = self
            .depth_table_view
            .iter()
            .filter(|(key, _)| **key > depth)
            .filter(|(_, val)| val.contains_key(name))
            .count();
        if !(inserted || count != 0) {
            return;
        }
        self.storage.push(symbol);
        let idx = self.storage.len() - 1;
        let symbol = &self.storage[idx];
        let name = match symbol {
            SymbolType::Type(type_) => {
                self.type_table_view
                    .entry(type_.name.clone())
                    .or_default()
                    .push(idx);
                type_.name.as_str()
            }
            SymbolType::Variable(variable) => {
                self.variable_table_view
                    .entry(variable.name.clone())
                    .or_default()
                    .push(idx);
                variable.name.as_str()
            }
        };
        self.depth_table_view
            .entry(depth)
            .or_default()
            .insert(name.to_string(), idx);
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
