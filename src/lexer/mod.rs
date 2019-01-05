use std::ffi::OsString;
use nom::types::CompleteStr;

mod tokens;
mod error;

#[cfg(test)]
mod tests;

pub use self::error::InvalidToken;
pub use self::tokens::Token;

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct Lexer<'input> {
    pub source_filename: OsString,
    pub source: &'input str,

    /* the current offset (in bytes) from the beginning of the file */
    offset: usize
}

impl<'input> Lexer<'input> {
    pub fn new(source_filename: OsString, source: &'input str) -> Lexer<'input> {
        Lexer {
            source_filename,
            source,
            offset: 0
        }
    }

    pub fn advance(&mut self) -> Option<Spanned<Token<'input>, usize, InvalidToken>> {
        if self.offset >= self.source.len() {
            return None;
        }
        let s = &self.source[self.offset..].trim_left();
        let offset = self.offset + ((self.source.len() - self.offset) - s.len());
        let slice = CompleteStr(&self.source[offset..]);

        if slice.len() == 0 {
            return None;
        }

        match token(slice) {
            Ok((input,token)) => {
                let token_len = s.len() - input.len();
                let new_offset = offset + token_len;
                self.offset = new_offset;
                return Some(Ok((offset, token, token_len)));
            },
            Err(_err) => {
                // invalid (or incomplete) token
                return Some(Err(InvalidToken::from_offset(self.source_filename.clone(), &self.source, offset)));
            }
        };
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>, usize, InvalidToken>;
    
    /// Get the next token from the source file
    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}

// a macro to generate a named parser from a string literal to a token
macro_rules! recognize_tag {
    ($name: ident, $s: tt, $token: expr) => {
        named!($name(CompleteStr) -> Token, do_parse!(
            tag!($s) >>
            ($token)
        ));
    };
}

macro_rules! recognize_char {
    ($name: ident, $c: tt, $token: expr) => {
        named!($name(CompleteStr) -> Token, do_parse!(
            char!($c) >>
            ($token)
        ));
    };
}

named!(token(CompleteStr) -> Token, alt!(
    punctuation | ident | keyword | integer_literal
));

named!(punctuation(CompleteStr) -> Token, alt!(
    l_paren | r_paren | l_brace | r_brace | l_bracket | r_bracket | semicolon |
    comma | ellipsis | dot | star | arrow
));

named!(keyword(CompleteStr) -> Token, alt!(
    _return | inline | typedef | _extern | _static | auto |
    register | _const | restrict | volatile | void | char |
    short | int | long | float | double | signed | unsigned
));

/* punctuation */
recognize_char!(l_paren, '(', Token::LParen);
recognize_char!(r_paren, ')', Token::RParen);
recognize_char!(l_brace, '{', Token::LBrace);
recognize_char!(r_brace, '}', Token::RBrace);
recognize_char!(l_bracket, '[', Token::LBracket);
recognize_char!(r_bracket, ']', Token::RBracket);
recognize_char!(semicolon, ';', Token::Semicolon);
recognize_char!(comma, ',', Token::Comma);
recognize_tag!(ellipsis, "...", Token::Ellipsis);
recognize_char!(dot, '.', Token::Dot);
recognize_char!(star, '*', Token::Star);
recognize_tag!(arrow, "->", Token::Arrow);

recognize_tag!(_return, "return", Token::Return);

/* function specifier */
recognize_tag!(inline, "inline", Token::Inline);

/* storage class specifiers */
recognize_tag!(typedef, "typedef", Token::Typedef);
recognize_tag!(_extern, "extern", Token::Extern);
recognize_tag!(_static, "static", Token::Static);
recognize_tag!(auto, "auto", Token::Auto);
recognize_tag!(register, "register", Token::Register);

/* type qualifier */
recognize_tag!(_const, "const", Token::Const);
recognize_tag!(restrict, "restrict", Token::Restrict);
recognize_tag!(volatile, "volatile", Token::Volatile);

/* type specifier */
recognize_tag!(void, "void", Token::Void);
recognize_tag!(char, "char", Token::Char);
recognize_tag!(short, "short", Token::Short);
recognize_tag!(int, "int", Token::Int);
recognize_tag!(long, "long", Token::Long);
recognize_tag!(float, "float", Token::Float);
recognize_tag!(double, "double", Token::Double);
recognize_tag!(signed, "signed", Token::Signed);
recognize_tag!(unsigned, "unsigned", Token::Unsigned);

/* the identifier can not be a reserved word */
named!(ident(CompleteStr) -> Token, do_parse!(
    peek!(alt!(nom::alpha | tag!("_"))) >>
    ident: verify!(take_while1!(|c: char| c.is_alphanumeric() || c == '_'), |s: CompleteStr| !is_keyword(&s)) >>
    (Token::Identifier(&ident))
));

fn is_keyword<'a>(s: &'a str) -> bool {
    const KEYWORDS: [&'static str; 19] = ["return", "inline", "typedef", "extern", "static", "auto",
        "register", "const", "restrict", "volatile", "void", "char", "short", "int", "long", "float",
        "double", "signed", "unsigned"];
    KEYWORDS.iter().any(|e| e == &s)
}

named!(integer_literal(CompleteStr) -> Token, alt!(
    hex_integer_literal | decimal_integer_literal
));

named!(decimal_integer_literal(CompleteStr) -> Token, do_parse!(
    literal: take_while1!(|c: char| c.is_digit(10)) >>
    (Token::IntLiteral(&literal))
));

named!(hex_integer_literal(CompleteStr) -> Token, do_parse!(
    literal: recognize!(do_parse!(
        tag!("0x") >>
        take_while1!(|c: char| c.is_digit(10)) >>
        (())
    )) >>
    (Token::IntLiteral(&literal))
));
