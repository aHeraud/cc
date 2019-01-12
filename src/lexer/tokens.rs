use crate::parser::ast::Integer;

#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    /* punctuation */
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Star,           /* can be multiplication or a pointer depending on context */
    Ellipsis,       /* "...", used to indicate that a function can take a variable number of parameters */
    Dot,            /* can be struct member access, or can be 3 dots in a row for a variadic function declarator */
    Arrow,          /* "->" operator for member access */
    Increment,      /* ++ */
    Decrement,      /* -- */
    Ampersand,      /* & */
    Plus,
    Minus,
    Tilde,
    Exclamation,
    Slash,
    Colon,
    Question,       /* ? */
    Modulo,         /* % */
    Shl,            /* << */
    Shr,            /* >> */
    LessThan,       /* < */
    GreaterThan,    /* > */
    LessThanOrEqualTo,      /* <= */
    GreaterThanOrEqualTo,   /* >= */
    Equality,       /* == */
    NotEqual,       /* != */
    Caret,          /* ^ */
    VerticalBar,    /* | */
    AndAnd,         /* && */
    OrOr,           /* || */

    /* Assignment expressions */
    Equal,          /* = */
    MultEq,         /* *= */
    DivEq,          /* /= */
    ModEq,          /* %= */
    PlusEq,         /* += */
    MinusEq,        /* -= */
    ShlEq,          /* <<= */
    ShrEq,          /* >>= */
    AndEq,          /* &= */
    XorEq,          /* ^= */
    OrEq,           /* |= */

    /* Keywords */
    Goto,
    Continue,
    Break,
    Return,
    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    Do,
    For,
    Inline,
    Typedef,
    Extern,
    Static,
    Auto,
    Register,
    Const,
    Restrict,
    Volatile,
    Struct,
    Union,
    Enum,
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    SizeOf,
    //_Bool,
    //_Complex,

    /* identifiers */
    Identifier(&'a str),

    /* An integer literal, can be in decimal, hex (0x), octal (0), or binary (0b) */
    IntLiteral(Integer),

    /* a floating point literal, consists of an optional whole number part, followed by '.', and
       then an optional fraction part, followed by an optional exponent part, 
       followed by an optional suffix (one of ['f', 'l', 'F', 'L'])
    */
    FloatLiteral(&'a str),

    StringLiteral(&'a str)
}
