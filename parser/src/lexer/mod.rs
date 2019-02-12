use std::ffi::OsString;
use std::collections::HashSet;
use std::rc::Rc;

use nom::types::CompleteStr;

mod tokens;
mod error;
mod integer_literals;

#[cfg(test)]
mod tests;

use self::integer_literals::integer_literal;
pub use error::InvalidToken;
pub use self::tokens::Token;
use ast::Location;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        ["goto", "continue", "break", "return", "inline", "typedef",
        "extern", "static", "auto", "register", "const", "restrict",
        "volatile", "void", "char", "short", "int", "long", "float",
        "double", "signed", "unsigned", "sizeof", "struct", "union",
        "enum", "switch", "else", "case", "default", "while", "for",
        "do", "if"].iter().cloned().collect()
    };
}

pub type Spanned<Token, Loc, Error> = Result<(Loc, Token, Loc), Error>;

pub struct Lexer<'input> {
    pub source_filename: Rc<OsString>,
    pub source: &'input str,

    /// the current offset (in bytes) from the beginning of the file
    offset: usize,

    /// The current line (meaning, the next time we try to lex a token we will start on this line)
    line: usize,

    /// The offset from the beginning of the file to the beginning of the current line.
    column: usize
}

impl<'input> Lexer<'input> {
    pub fn new(source_filename: OsString, source: &'input str) -> Lexer<'input> {
        Lexer {
            source_filename: Rc::new(source_filename),
            source,
            offset: 0,
            line: 1,
            column: 1
        }
    }

    pub fn advance(&mut self) -> Option<Spanned<Token<'input>, Location, InvalidToken>> {
        if self.offset >= self.source.len() {
            return None;
        }
        let s = &self.source[self.offset..].trim_left();
        let new_offset = self.offset + ((self.source.len() - self.offset) - s.len());
        self.update_location(self.offset, new_offset);
        self.offset = new_offset;
        let slice = CompleteStr(&self.source[self.offset..]);

        if slice.len() == 0 {
            return None;
        }

        let start = self.location();
        match token(slice) {
            Ok((input,token)) => {
                let token_len = s.len() - input.len();
                let new_offset = self.offset + token_len;
                self.update_location(self.offset, new_offset);
                self.offset = new_offset;
                return Some(Ok((start, token, self.location())));
            },
            Err(_err) => {
                // invalid (or incomplete) token
                return Some(Err(InvalidToken::from_offset(self.location(), self.source)));
            }
        };
    }

    /// Updates the line and column fields by counting newlines that have recently been lexed
    fn update_location(&mut self, old_offset: usize, new_offset: usize) {
        let s = &self.source[old_offset..new_offset];
        for c in s.chars() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            else {
                self.column += 1;
            }
        }
    }

    fn location(&self) -> Location {
        Location::new(self.source_filename.clone(), self.line, self.column, self.offset)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>, Location, InvalidToken>;
    
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
    punctuation | string_literal | ident | keyword | integer_literal
));

named!(punctuation(CompleteStr) -> Token, alt!(
    alt!( /* 3 char long tokens */
        shl_eq | shr_eq
    ) |
    alt!( /* 2 char long tokens */
        equality | not_equal | less_than_or_equal_to |
        greater_than_or_equal_to | increment| decrement |
        shl | shr | and_and | or_or | mult_eq | div_eq |
        mod_eq | plus_eq | minus_eq | and_eq | xor_eq |
        or_eq
    ) |
    alt!( /* single char tokens */
        l_paren | r_paren | l_brace | r_brace | l_bracket |
        r_bracket | semicolon | comma | ellipsis | dot |
        star | arrow | ampersand | plus | minus | tilde |
        exclamation | slash | modulo | less_than | greater_than |
        caret | vertical_bar | colon | question_mark | equal
    )
));

named!(keyword(CompleteStr) -> Token, alt!(
    jump_keywords | inline | typedef |
    _extern | _static | auto | register | _const | restrict |
    volatile | sizeof | type_keywords |
    _if | _else | _switch | case | default | loop_keywords
));

named!(type_keywords(CompleteStr) -> Token, alt!(
    void | char | short | int | long | float | double |
    signed | unsigned | _struct | _union | _enum
));

named!(jump_keywords(CompleteStr) -> Token, alt!(
    _continue | _break | _goto | _return
));

named!(loop_keywords(CompleteStr) -> Token, alt!(
    _while | _for | _do
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
recognize_tag!(increment, "++", Token::Increment);
recognize_tag!(decrement, "--", Token::Decrement);
recognize_char!(ampersand, '&', Token::Ampersand);
recognize_char!(plus, '+', Token::Plus);
recognize_char!(minus, '-', Token::Minus);
recognize_char!(tilde, '~', Token::Tilde);
recognize_char!(exclamation, '!', Token::Exclamation);
recognize_char!(slash, '/', Token::Slash);
recognize_char!(modulo, '%', Token::Modulo);
recognize_char!(caret, '^', Token::Caret);
recognize_char!(vertical_bar, '|', Token::VerticalBar);
recognize_char!(colon, ':', Token::Colon);
recognize_char!(question_mark, '?', Token::Question);
recognize_tag!(shl, "<<", Token::Shl);
recognize_tag!(shr, ">>", Token::Shr);
recognize_char!(less_than, '<', Token::LessThan);
recognize_char!(greater_than, '>', Token::GreaterThan);
recognize_tag!(less_than_or_equal_to, "<=", Token::LessThanOrEqualTo);
recognize_tag!(greater_than_or_equal_to, ">=", Token::GreaterThanOrEqualTo);
recognize_tag!(equality, "==", Token::Equality);
recognize_tag!(not_equal, "!=", Token::NotEqual);
recognize_tag!(and_and, "&&", Token::AndAnd);
recognize_tag!(or_or, "||", Token::OrOr);

/* assignment operators */
recognize_char!(equal, '=', Token::Equal);
recognize_tag!(mult_eq, "*=", Token::MultEq);
recognize_tag!(div_eq, "/=", Token::DivEq);
recognize_tag!(mod_eq, "%=", Token::ModEq);
recognize_tag!(plus_eq, "+=", Token::PlusEq);
recognize_tag!(minus_eq, "-=", Token::MinusEq);
recognize_tag!(shl_eq, "<<=", Token::ShlEq);
recognize_tag!(shr_eq, ">>=", Token::ShrEq);
recognize_tag!(and_eq, "&=", Token::AndEq);
recognize_tag!(xor_eq, "^=", Token::XorEq);
recognize_tag!(or_eq, "|=", Token::OrEq);

/* jump keywords */
recognize_tag!(_goto, "goto", Token::Goto);
recognize_tag!(_continue, "continue", Token::Continue);
recognize_tag!(_break, "break", Token::Break);
recognize_tag!(_return, "return", Token::Return);

recognize_tag!(_if, "if", Token::If);
recognize_tag!(_else, "else", Token::Else);
recognize_tag!(_switch, "switch", Token::Switch);

recognize_tag!(case, "case", Token::Case);
recognize_tag!(default, "default", Token::Default);

/* loop keywords */
recognize_tag!(_while, "while", Token::While);
recognize_tag!(_do, "do", Token::Do);
recognize_tag!(_for, "for", Token::For);

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

/* struct or union */
recognize_tag!(_struct, "struct", Token::Struct);
recognize_tag!(_union, "union", Token::Union);

recognize_tag!(_enum, "enum", Token::Enum);

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

// misc
recognize_tag!(sizeof, "sizeof", Token::SizeOf);

/* the identifier can not be a reserved word */
named!(ident(CompleteStr) -> Token, do_parse!(
    peek!(alt!(nom::alpha | tag!("_"))) >>
    ident: verify!(take_while1!(|c: char| c.is_alphanumeric() || c == '_'), |s: CompleteStr| !is_keyword(&s)) >>
    (Token::Identifier(&ident))
));

fn is_keyword<'a>(s: &'a str) -> bool {
    KEYWORDS.contains(s)
}

named!(string_literal(CompleteStr) -> Token, do_parse!(
    wide: opt!(char!('L')) >>
    char!('"') >>
    s: recognize!(many0!(s_char)) >>
    char!('"') >>
    (Token::StringLiteral{ wide: wide.is_some(), contents: &s })
));

named!(s_char(CompleteStr) -> CompleteStr, alt!(
        recognize!(none_of!("\"\\\n")) |
        escape_sequence
));

named!(escape_sequence(CompleteStr) -> CompleteStr, alt!(
    /* simple escape sequence */
    alt!(
        tag!("\\'") |
        tag!("\\\"") |
        tag!("\\?") |
        tag!("\\a") |
        tag!("\\b") |
        tag!("\\f") |
        tag!("\\n") |
        tag!("\\r") |
        tag!("\\t") |
        tag!("\\v")
    ) |
    /* octal escape sequence */
    recognize!(do_parse!(
        tag!("\\") >>
        take_while1!(|c: char| c.is_digit(8)) >>
        ()
    )) |
    /* hexadecimal escape sequence */
    recognize!(do_parse!(
        tag!("\\x") >>
        take_while1!(|c: char| c.is_digit(16)) >>
        ()
    )) |
    /* universal character name */
    recognize!(do_parse!(
        tag!("\\u") >>
        take_while1!(|c: char| c.is_digit(16)) >>
        ()
    ))
));
