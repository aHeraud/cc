use std::collections::HashMap;

use types::*;
use ast::Location;
use errors::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VariableID(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LabelID(i32);

/// The symbol table is implemented as a stack of hash maps.
/// Each lexical scope level is an element on the stack.
pub struct SymbolTable {
    scope_stack: Vec<ScopeLevel>
}

impl SymbolTable {
     /// Creates a new symbol table.
     /// The scope stack will contain a single empty scope level, the global scope.
    pub fn new() -> SymbolTable {
        SymbolTable {
            scope_stack: vec![ ScopeLevel::new(ScopeType::Global) ]
        }
    }

    pub fn lookup_identifier(&self, name: &String) -> Option<&(OrdinaryIdentifier, Location)> {
        for level in (0..self.scope_stack.len()).rev() {
            let ident = self.scope_stack[level].ordinary_identifiers.get(name);
            if ident.is_some() {
                return ident;
            }
        }
        None
    }

    pub fn insert_typedef(&mut self, name: &String, value: QualifiedType, location: Location) -> Result<(), TypedefRedefinitionError> {
        unimplemented!()
    }

    pub fn insert_variable(&mut self, name: &String, value: Type, location: Location) -> Result<(), VariableRedefinitionError> {
        if let Some((ident, old_location)) = self.scope_stack.last().unwrap().ordinary_identifiers.get(name) {
            if let OrdinaryIdentifier::Variable(old_type) = ident {
                // it's illegal to redeclare a variable inside the same scope level
                Err(VariableRedefinitionError::new(name.clone(), location, old_location.clone()))
            }
            else {
                // it is illegal to redeclare an identifier as a different kind of symbol within the same scope level
                //TODO: error
                panic!()
            }
        }
        else {
            self.scope_stack.last_mut().unwrap().ordinary_identifiers.insert(name.clone(), (OrdinaryIdentifier::Variable(value), location));
            Ok(())
        }
    }

    pub fn scope_enter(&mut self, scope_type: ScopeType) {
        self.scope_stack.push(ScopeLevel::new(scope_type));
    }

    pub fn scope_leave(&mut self) {
        // you can't leave the global scope
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }
}

pub struct ScopeLevel {
    scope_type: ScopeType,
    ordinary_identifiers: HashMap<String, (OrdinaryIdentifier, Location)>, // variables, functions, and typedefs share the same namespace
    tags: HashMap<String, Tag>,
    labels: HashMap<String, ()>, //TODO
    structs: HashMap<StructID,Struct>,
    unions: HashMap<UnionID, Union>,
    enums: HashMap<EnumID, Enum>
}

impl ScopeLevel {
    pub fn new(scope_type: ScopeType) -> ScopeLevel {
        ScopeLevel {
            scope_type,
            ordinary_identifiers: HashMap::new(),
            tags: HashMap::new(),
            labels: HashMap::new(),
            structs: HashMap::new(),
            unions: HashMap::new(),
            enums: HashMap::new()
        }
    }

    pub fn level_type(&self) -> &ScopeType {
        &self.scope_type
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ScopeType {
    Global,
    Function,
    Block
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Struct(StructID),
    Enum(EnumID),
    Union(UnionID)
}

/// Variable identifiers, functions, and typedefs share a namespace
#[derive(Debug)]
pub enum OrdinaryIdentifier {
    Variable(QualifiedType, StorageClass),
    EnumVariant(i32),
    Function(Option<Vec<Type>>, Type),
    Typedef(QualifiedType)
}
