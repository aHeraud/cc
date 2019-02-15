extern crate ast;
extern crate lexer;
extern crate lalrpop_util;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::convert::From;

use lalrpop_util::ParseError;

use ast::{Node, Location};

#[derive(Debug)]
pub enum CompilationError<'a> {
    ParseError(ParseError<Location, lexer::Token<'a>, lexer::InvalidToken>),
    TypedefRedefinition(TypedefRedefinitionError),
    InvalidStorageClassSpecifierCombination(InvalidStorageClassSpecifierCombination),
    InvalidTypeSpecifierCombination(InvalidTypeSpecifierCombination)
}

impl<'a> Display for CompilationError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompilationError::ParseError(inner) => inner.fmt(f),
            CompilationError::TypedefRedefinition(inner) => inner.fmt(f),
            CompilationError::InvalidStorageClassSpecifierCombination(inner) => inner.fmt(f),
            CompilationError::InvalidTypeSpecifierCombination(inner) => inner.fmt(f)
        }
    }
}

impl<'a> Error for CompilationError<'a> {}

impl<'a> From<ParseError<Location, lexer::Token<'a>, lexer::InvalidToken>> for CompilationError<'a> {
    fn from(error: ParseError<Location, lexer::Token<'a>, lexer::InvalidToken>) -> Self {
        CompilationError::ParseError(error)
    }
}

impl<'a> From<TypedefRedefinitionError> for CompilationError<'a> {
    fn from(error: TypedefRedefinitionError) -> Self {
        CompilationError::TypedefRedefinition(error)
    }
}

impl<'a> From<InvalidStorageClassSpecifierCombination> for CompilationError<'a> {
    fn from(error: InvalidStorageClassSpecifierCombination) -> Self {
        CompilationError::InvalidStorageClassSpecifierCombination(error)
    }
}

impl<'a> From<InvalidTypeSpecifierCombination> for CompilationError<'a> {
    fn from(error: InvalidTypeSpecifierCombination) -> Self {
        CompilationError::InvalidTypeSpecifierCombination(error)
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

#[derive(Debug)]
pub struct InvalidTypeSpecifierCombination {
    specifier_list: Node<ast::DeclarationSpecifiers>,
    specifier: Node<ast::TypeSpecifier>,
    incompatible_previous_specifier: ast::TypeSpecifier
}

impl InvalidTypeSpecifierCombination {
    pub fn new(specifier_list: Node<ast::DeclarationSpecifiers>, specifier: Node<ast::TypeSpecifier>, incompatible_previous_specifier: ast::TypeSpecifier) -> InvalidTypeSpecifierCombination {
        InvalidTypeSpecifierCombination {
            specifier_list,
            specifier,
            incompatible_previous_specifier
        }
    }
}

impl Display for InvalidTypeSpecifierCombination {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: error: {} type specifier incompatible with previous '{}' type specifier", self.specifier.start, self.specifier.value, self.incompatible_previous_specifier)
    }
}

impl Error for InvalidTypeSpecifierCombination { }


#[derive(Debug)]
pub struct InvalidStorageClassSpecifierCombination {
    specifier_list: Node<ast::DeclarationSpecifiers>,
    specifier: Node<ast::StorageClassSpecifier>,
    incompatible_previous_specifier: ast::StorageClassSpecifier
}

impl InvalidStorageClassSpecifierCombination {
    pub fn new(specifier_list: Node<ast::DeclarationSpecifiers>, specifier: Node<ast::StorageClassSpecifier>, incompatible_previous_specifier: ast::StorageClassSpecifier) -> InvalidStorageClassSpecifierCombination {
        InvalidStorageClassSpecifierCombination {
            specifier_list,
            specifier,
            incompatible_previous_specifier
        }
    }
}

impl Display for InvalidStorageClassSpecifierCombination {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: error: {} storage class specifier can't be combined with previous '{}' storage class specifier", self.specifier.start, self.specifier.value, self.incompatible_previous_specifier)
    }
}
