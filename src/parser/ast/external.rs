use super::*;

pub type TranslationUnit = Vec<ExternalDeclaration>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalDeclaration {
    FunctionDefinition(())
}