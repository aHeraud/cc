use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition {
    pub declaration_specifiers: DeclarationSpecifiers,
    pub declarator: Declarator,
    //pub declaration_list: DeclarationList,
    pub compound_statement: CompoundStatement
}

//#[derive(Debug, Clone, PartialEq, Eq)]
//pub struct Pointer {
//    qualifiers: Vec<TypeQualifier>,
//    inner: Option<Box<Pointer>>
//}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarator {
    pub direct_declarator: DirectDeclarator
}

pub type DirectDeclarator = Vec<DirectDeclaratorPart>;

// TODO: figure out array declarators
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectDeclaratorPart {
    Identifier(String),
    Parens(Box<Declarator>),
    ParameterTypeList(ParameterTypeList),
    IdentifierList(IdentifierList)
}

pub type DeclarationSpecifiers = Vec<DeclarationSpecifier>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationSpecifier {
    StorageClassSpecifier(StorageClassSpecifier),
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier),
    FunctionSpecifier(FunctionSpecifier)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionSpecifier {
    Inline
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    Auto,
    Register
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeQualifier {
    Const,
    Restrict,
    Volatile
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSpecifier {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Unsigned,
    Signed
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterTypeList {
    pub parameter_list: Vec<ParameterDeclaration>,
    pub variadic: bool
}

impl ParameterTypeList {
    pub fn new(declarations: ParameterList, variadic: bool) -> ParameterTypeList {
        ParameterTypeList { parameter_list: declarations, variadic }
    }
}

pub type ParameterList = Vec<ParameterDeclaration>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterDeclarator {
    Declarator(Box<Declarator>),
    AbstractDeclarator(Option<Box<AbstractDeclarator>>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration {
    pub declaration_specifier_list: DeclarationSpecifiers,
    pub declarator: ParameterDeclarator
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractDeclarator {
    pub direct_abstract_declarator: DirectAbstractDeclarator
}

impl AbstractDeclarator {
    pub fn new(direct_abstract_declarator: DirectAbstractDeclarator) -> AbstractDeclarator {
        AbstractDeclarator {
            direct_abstract_declarator
        }
    }
}

pub type DirectAbstractDeclarator = Vec<DirectAbstractDeclaratorPart>;

pub type IdentifierList = Vec<String>;

// TODO: array
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectAbstractDeclaratorPart {
    Parens(Box<AbstractDeclarator>),
    // Arrray(Option<AssignmentExpression>)  // assignment expression should be an integer here?
    VLA,  // this has the form of "[*]"
    ParameterTypeList(Box<ParameterTypeList>)
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum IntType {
//     Char,
//     Short,
//     Int,
//     Long
// }
