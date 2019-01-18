use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub declaration_specifiers: DeclarationSpecifiers,
    pub init_declarator_list: Option<InitDeclaratorList>
}

impl Declaration {
    pub fn new(declaration_specifiers: DeclarationSpecifiers, init_declarator_list: Option<InitDeclaratorList>) -> Declaration {
        Declaration {
            declaration_specifiers,
            init_declarator_list
        }
    }
}

pub type InitDeclaratorList = Vec<InitDeclarator>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitDeclarator {
    pub declarator: Declarator,
    pub initializer: Option<Initializer>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declarator {
    pub pointer: Option<Vec<Pointer>>,
    pub direct_declarator: DirectDeclarator
}

pub type DirectDeclarator = Vec<DirectDeclaratorPart>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectDeclaratorPart {
    Identifier(String),
    Parens(Box<Declarator>),
    Array(Option<AssignmentExpression>), /* discards static keyword and type qualifier list preceeding the optional assignment expression */
    VLA, /* [*] */
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
    Signed,
    StructOrUnionSpecifier(StructOrUnionSpecifier),
    EnumSpecifier(EnumSpecifier),
    Typedef(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructOrUnionSpecifier {
    Partial { kind: StructOrUnion, identifier: String },
    Complete { kind: StructOrUnion, identifier: Option<String>, declaration_list: StructDeclarationList }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructOrUnion {
    Struct,
    Union
}

pub type StructDeclarationList = Vec<StructDeclaration>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDeclaration {
    pub specification_qualifier_list: SpecifierQualifierList,
    pub struct_declaration_list: StructDeclaratorList
}

impl StructDeclaration {
    pub fn new(specification_qualifier_list: SpecifierQualifierList, struct_declaration_list: StructDeclaratorList) -> StructDeclaration {
        StructDeclaration {
            specification_qualifier_list,
            struct_declaration_list
        }
    }
}

pub type SpecifierQualifierList = Vec<SpecifierQualifier>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecifierQualifier {
    TypeSpecifier(TypeSpecifier),
    TypeQualifier(TypeQualifier)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName {
    specifier_qualifier_list: SpecifierQualifierList,
    abstract_declarator: Option<AbstractDeclarator>
}

impl TypeName {
    pub fn new(l: SpecifierQualifierList, dec: Option<AbstractDeclarator>) -> TypeName {
        TypeName {
            specifier_qualifier_list: l,
            abstract_declarator: dec
        }
    }
}

pub type StructDeclaratorList = Vec<StructDeclarator>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructDeclarator {
    Field(Declarator),
    BitField(Option<Declarator>, ConstantExpression)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumSpecifier {
    Complete { identifier: Option<String>, enumerator_list: EnumeratorList },
    Partial { identifier: String }
}

pub type EnumeratorList = Vec<Enumerator>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumerator {
    pub identifier: String,
    pub value: Option<ConstantExpression>
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
pub struct Pointer {
    pub qualifiers: Vec<TypeQualifier>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractDeclarator {
    Pointer(Vec<Pointer>),
    DirectAbstractDeclarator { pointer: Option<Vec<Pointer>>, direct_abstract_declarator: DirectAbstractDeclarator }
}

pub type DirectAbstractDeclarator = Vec<DirectAbstractDeclaratorPart>;

pub type IdentifierList = Vec<String>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectAbstractDeclaratorPart {
    Parens(Box<AbstractDeclarator>),
    Array(Option<AssignmentExpression>), // type qualifiers appearing before the assignment expression are discarded
    VLA,  // this has the form of "[*]"
    ParameterTypeList(Box<ParameterTypeList>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Initializer {
    AssignmentExpression(AssignmentExpression),
    InitializerList(InitializerList)
}

pub type InitializerList = Vec<InitializerListItem>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializerListItem {
    pub designator: Option<DesignatorList>,
    pub initializer: Box<Initializer>
}

pub type DesignatorList = Vec<Designator>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Designator {
    Index(ConstantExpression),
    Field(String)
}