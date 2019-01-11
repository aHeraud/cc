use super::*;

pub type CompoundStatement = Vec<BlockItem>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Box<Statement>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Jump(JumpStatement),
    Labeled(LabeledStatement),
    Compound(CompoundStatement),
    /// Expression Statement = [ Expression ] ";"
    /// The expression is optional here, which allows you do do something like
    /// write a labeled statement with no body (`label: ;`).
    Expression(Option<Expression>),
    Selection(SelectionStatement),
    Iteration(IterationStatement)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabeledStatement {
    Statement{ label: String, statement: Box<Statement> },
    Case { value: ConstantExpression, body: Box<Statement> },
    DefaultCase { body: Box<Statement> }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionStatement {
    If { condition: Expression, body: Box<Statement>, else_clause: Option<Box<Statement>> },
    Switch { condition: Expression, body: Box<Statement> }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IterationStatement {
    While { condition: Expression, body: Box<Statement> },
    DoWhile { body: Box<Statement>, condition: Expression },
    ForA{ expr1: Option<Expression>, condition: Option<Expression>, expr3: Option<Expression>, body: Box<Statement> },
    ForB{ clause1: Declaration, condition: Option<Expression>, expr3: Option<Expression>, body: Box<Statement> }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JumpStatement {
    Goto(String),
    Continue,
    Break,
    Return(Option<Box<Expression>>)
}
