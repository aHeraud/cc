use super::*;

pub type CompoundStatement = Vec<BlockItem>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockItem {
    Statement(Statement)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Jump(JumpStatement)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JumpStatement {
    Return(Option<Box<Expression>>)
}

