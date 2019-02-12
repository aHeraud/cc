extern crate ast;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::convert::From;

use ast::{Node, Location};

#[derive(Debug)]
pub enum CompilationError {
    TypedefRedefinition(TypedefRedefinitionError),
}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompilationError::TypedefRedefinition(inner) => inner.fmt(f)
        }
    }
}

impl Error for CompilationError {}

impl From<TypedefRedefinitionError> for CompilationError {
    fn from(error: TypedefRedefinitionError) -> Self {
        CompilationError::TypedefRedefinition(error)
    }
}

#[derive(Debug)]
pub struct TypedefRedefinitionError {
    identifier: String,
    location: Location,
    previous_definition: Location
}

impl TypedefRedefinitionError {
    pub fn new(identifier: String, location: Location, previous_definition: Location) -> TypedefRedefinitionError {
        TypedefRedefinitionError {
            identifier,
            location,
            previous_definition
        }
    }
}

impl Display for TypedefRedefinitionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: error: conflicting types for '{}'\nprevious definition at {}", self.location, self.identifier, self.previous_definition)
    }
}

impl Error for TypedefRedefinitionError {}

pub struct VariableRedefinitionError {
    identifier: String,
    location: Location,
    previous_location: Location
}

//TODO: additional information for conflicting types?
impl VariableRedefinitionError{
    pub fn new(identifier: String, location: Location, previous_location: Location) -> VariableRedefinitionError {
        VariableRedefinitionError {
            identifier,
            location,
            previous_location
        }
    }
}

impl Display for VariableRedefinitionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: error: redeclaration of '{}'\nprevious definition at {}", self.location, self.identifier, self.previous_location)
    }
}

#[derive(Debug)]
pub struct MultipleStorageClassSpecifiersInDeclarationError {
    declaration: Node<ast::Declaration>
}

impl Display for MultipleStorageClassSpecifiersInDeclarationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: error: declaration contains multiple storage class specifiers", self.declaration.start)
    }
}

impl Error for MultipleStorageClassSpecifiersInDeclarationError { }