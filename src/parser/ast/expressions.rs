#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constant {
    Integer(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryExpression {
    Identifier(String),
    Constant(Constant),
    StringLiteral(String),
    Parens(Box<Expression>)
}

pub type PostfixExpression = Vec<PostfixExpressionPart>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostfixExpressionPart {
    PrimaryExpression(PrimaryExpression),
    ArrayAccess(Box<Expression>),
    ArgumentExpressionList(ArgumentExpressionList),
    MemberAccess(String), // member access with dot operator
    PointerMemberAccess(String), // member access with arrow operator
    Increment, // postfix ++
    Decrement, // postfix --
    //TypeInitializerList{type_name: TypeName, initializer_list: InitializerList} // TODO: (what is the definition for an initializer-list???)
}

pub type ArgumentExpressionList = Vec<AssignmentExpression>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpression {
    PostfixExpression(PostfixExpression),
    Increment(Box<UnaryExpression>),
    Decrement(Box<UnaryExpression>),
    UnaryOperator((UnaryOperator, CastExpression)),
    SizeOfExpr(Box<UnaryExpression>),
    //SizeOfType(type?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    AddressOf, // &a
    Indirection, // *a
    Plus, // +a (something about integer promotion)
    Minus, // -a (additive inverse)
    BitwiseNot, // ~a
    LogicalNot // !a
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CastExpression {
    UnaryExpression(Box<UnaryExpression>),
    //Cast{ type_name: TypeName, cast_expression: Box<CastExpression> } // what is the definition of a type name
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplicativeExpression {
    CastExpression(Box<CastExpression>),
    Mult((Box<MultiplicativeExpression>, Box<CastExpression>)),
    Div((Box<MultiplicativeExpression>, Box<CastExpression>)),
    Mod((Box<MultiplicativeExpression>, Box<CastExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdditiveExpression {
    MultiplicativeExpression(Box<MultiplicativeExpression>),
    Add((Box<AdditiveExpression>, Box<MultiplicativeExpression>)),
    Sub((Box<AdditiveExpression>, Box<MultiplicativeExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftExpression {
    AdditiveExpression(Box<AdditiveExpression>),
    Shl((Box<ShiftExpression>, Box<AdditiveExpression>)),
    Shr((Box<ShiftExpression>, Box<AdditiveExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalExpression {
    ShiftExpression(Box<ShiftExpression>),
    LessThan((Box<RelationalExpression>, Box<ShiftExpression>)),
    GreaterThan((Box<RelationalExpression>, Box<ShiftExpression>)),
    LessThanOrEqualTo((Box<RelationalExpression>, Box<ShiftExpression>)),
    GreaterThanOrEqualTo((Box<RelationalExpression>, Box<ShiftExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EqualityExpression {
    RelationalExpression(Box<RelationalExpression>),
    Equals((Box<EqualityExpression>, Box<RelationalExpression>)),
    NotEquals((Box<EqualityExpression>, Box<RelationalExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AndExpression {
    EqualityExpression(Box<EqualityExpression>),
    And((Box<AndExpression>, Box<EqualityExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XorExpression {
    AndExpression(Box<AndExpression>),
    Xor((Box<XorExpression>, Box<AndExpression>)),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrExpression {
    XorExpression(Box<XorExpression>),
    /// inclusive or expression of the form `a | b`
    Or((Box<OrExpression>, Box<XorExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalAndExpression {
    OrExpression(Box<OrExpression>),
    /// logical and expression of the form `a && b`
    LogicalAnd((Box<LogicalAndExpression>, Box<OrExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalOrExpression {
    LogicalAndExpression(Box<LogicalAndExpression>),
    /// logical or expression of the form `a || b`
    LogicalOr((Box<LogicalOrExpression>, Box<LogicalAndExpression>))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalExpression {
    LogicalOrExpression(Box<LogicalOrExpression>),
    /// ternary expression of the form `condition ? true_expr : false_expr`
    ConditionalExpression{ condition: Box<LogicalOrExpression>, true_expr: Box<Expression>, false_expr: Box<ConditionalExpression>}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentExpression {
    ConditionalExpression(Box<ConditionalExpression>),
    Assignment{ lhs: Box<UnaryExpression>, op: AssignmentOperator, rhs: Box<AssignmentExpression> }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentOperator {
    Eq, // =
    MultEq, // *=
    DivEq, // /=
    ModEq, // %=
    PlusEq, // +=
    MinusEq, // -=
    ShlEq, // <<=
    ShrEq, // >>=
    AndEq, // &=
    XorEq, // ^=
    OrEq // |=
}

pub type Expression = Vec<AssignmentExpression>;

pub type ConstantExpression = ConditionalExpression;