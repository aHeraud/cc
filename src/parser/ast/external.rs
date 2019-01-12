use super::*;

pub type TranslationUnit = Vec<ExternalDeclaration>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    Declaration(Declaration)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition {
    pub declaration_specifiers: DeclarationSpecifiers,
    pub declarator: Declarator,
    pub declaration_list: DeclarationList,
    pub compound_statement: CompoundStatement
}

pub type DeclarationList = Vec<Declaration>;