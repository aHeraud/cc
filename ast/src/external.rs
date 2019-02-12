use super::*;

pub type TranslationUnit = Vec<Node<ExternalDeclaration>>;

#[derive(Debug, Clone)]
pub enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    Declaration(Declaration)
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub declaration_specifiers: DeclarationSpecifiers,
    pub declarator: Declarator,
    pub declaration_list: DeclarationList,
    pub compound_statement: CompoundStatement
}

pub type DeclarationList = Vec<Node<Declaration>>;
