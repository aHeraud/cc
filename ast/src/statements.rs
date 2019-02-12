use super::*;

pub type CompoundStatement = Vec<Node<BlockItem>>;

#[derive(Debug, Clone)]
pub enum BlockItem {
    Declaration(Declaration),
    Statement(Box<Statement>)
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum LabeledStatement {
    Statement{ label: String, statement: Box<Node<Statement>> },
    Case { value: Node<ConstantExpression>, body: Box<Node<Statement>> },
    DefaultCase { body: Box<Node<Statement>> }
}

#[derive(Debug, Clone)]
pub enum SelectionStatement {
    If { condition: Node<Expression>, body: Box<Node<Statement>>, else_clause: Option<Box<Node<Statement>>> },
    Switch { condition: Node<Expression>, body: Box<Node<Statement>> }
}

#[derive(Debug, Clone)]
pub enum IterationStatement {
    While { condition: Node<Expression>, body: Box<Node<Statement>> },
    DoWhile { body: Box<Node<Statement>>, condition: Node<Expression >},
    ForA{ expr1: Option<Node<Expression>>, condition: Option<Node<Expression>>, expr3: Option<Node<Expression>>, body: Box<Node<Statement>> },
    ForB{ clause1: Node<Declaration>, condition: Option<Node<Expression>>, expr3: Option<Node<Expression>>, body: Box<Node<Statement>> }
}

#[derive(Debug, Clone)]
pub enum JumpStatement {
    Goto(String),
    Continue,
    Break,
    Return(Option<Box<Expression>>)
}
