use ast::Integer;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug, Clone)]
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
    Bool,
    Complex,

    /* identifiers */
    Identifier(&'a str),

    TypedefType(&'a str),

    /* An integer literal, can be in decimal, hex (0x), octal (0), or binary (0b) */
    IntLiteral(Integer),

    /* a floating point literal, consists of an optional whole number part, followed by '.', and
       then an optional fraction part, followed by an optional exponent part, 
       followed by an optional suffix (one of ['f', 'l', 'F', 'L'])
    */
    FloatLiteral(&'a str),

    StringLiteral{ wide: bool, contents: &'a str }
}


impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Token::*;
        let s = match self {
            LParen => "(",
            RParen => ")",
            LBrace => "{",
            RBrace => "}",
            LBracket => "[",
            RBracket => "]",
            Semicolon => ";",
            Comma => ",",
            Star => "*",
            Ellipsis => "...",
            Dot => ".",
            Arrow => "->",
            Increment => "++",
            Decrement => "--",
            Ampersand => "&",
            Plus => "+",
            Minus => "-",
            Tilde => "~",
            Exclamation => "!",
            Slash => "/",
            Colon => ":",
            Question => "?",
            Modulo => "%",
            Shl => "<<",
            Shr => ">>",
            LessThan => "<",
            GreaterThan => ">",
            LessThanOrEqualTo => "<=",
            GreaterThanOrEqualTo => ">=",
            Equality => "==",
            NotEqual => "!=",
            Caret => "^",
            VerticalBar => "|",
            AndAnd => "&&",
            OrOr => "||",
            Equal => "=",
            MultEq => "*=",
            DivEq => "/=",
            ModEq => "%=",
            PlusEq => "+=",
            MinusEq => "-=",
            ShlEq => "<<=",
            ShrEq => ">>=",
            AndEq => "&=",
            XorEq => "^=",
            OrEq => "|=",
            Goto => "goto",
            Continue => "continue",
            Break => "break",
            Return => "return",
            If => "if",
            Else => "else",
            Switch => "switch",
            Case => "case",
            Default => "default",
            While => "while",
            Do => "do",
            For => "for",
            Inline => "inline",
            Typedef => "typedef",
            Extern => "extern",
            Static => "static",
            Auto => "auto",
            Register => "register",
            Const => "const",
            Restrict => "restrict",
            Volatile => "volatile",
            Struct => "struct",
            Union => "union",
            Enum => "enum",
            Void => "void",
            Char => "char",
            Short => "short",
            Int => "int",
            Long => "long",
            Float => "float",
            Double => "double",
            Signed => "signed",
            Unsigned => "unsigned",
            SizeOf => "sizeof",
            Bool => "_Bool",
            Complex => "_Complex",
            Identifier(ident) => ident,
            TypedefType(name) => name,
            IntLiteral(i) => return i.fmt(f),
            FloatLiteral(s) => s,
            StringLiteral{ wide, contents } => {
                if *wide {
                    return write!(f, "L\"{}\"", contents);
                }
                else {
                    return write!(f, "\"{}\"", contents);
                }
            }
        };
        write!(f, "{}", s)
    }
}