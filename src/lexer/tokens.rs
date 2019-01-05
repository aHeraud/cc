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
    Star, /* can be multiplication or a pointer depending on context */
    Ellipsis, /* "...", used to indicate that a function can take a variable number of parameters */
    Dot, /* can be struct member access, or can be 3 dots in a row for a variadic function declarator */
    Arrow, /* "->" operator for member access */

    /* Keywords */
    Return,
    Inline,
    Typedef,
    Extern,
    Static,
    Auto,
    Register,
    Const,
    Restrict,
    Volatile,
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    //_Bool,
    //_Complex,

    /* identifiers */
    Identifier(&'a str),

    /* An integer literal, can be in decimal, hex (0x), octal (0o), or binary (0b) */
    IntLiteral(&'a str),

    /* a floating point literal, consists of an optional whole number part, followed by '.', and
       then an optional fraction part, followed by an optional exponent part, 
       followed by an optional suffix (one of ['f', 'l', 'F', 'L'])
    */
    FloatLiteral(&'a str),

    StringLiteral(&'a str)
}
