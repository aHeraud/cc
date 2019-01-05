use std::ffi::OsString;

use super::{Lexer, Token};

macro_rules! lex_token_test {
    ($name: ident, $source: tt, $token: expr) => {
        #[test]
        fn $name() {
            let source: &'static str = $source;
            let mut lexer = Lexer::new(OsString::from(stringify!($name)), source);
            let (_, token, _) = lexer.next().unwrap().unwrap();
            assert_eq!(token, $token);
        }
    };
}

lex_token_test!(skip_ws, "\r\n\t  (", Token::LParen); // the lexer should discard whitespace

// punctuation
lex_token_test!(lparen, "(", Token::LParen);
lex_token_test!(rparen, ")", Token::RParen);
lex_token_test!(lbrace, "{", Token::LBrace);
lex_token_test!(rbrace, "}", Token::RBrace);
lex_token_test!(semicolon, ";", Token::Semicolon);
lex_token_test!(comma, ",", Token::Comma);
lex_token_test!(star, "*", Token::Star);
lex_token_test!(dot, ".", Token::Dot);
lex_token_test!(ellipsis, "...", Token::Ellipsis);
lex_token_test!(arrow, "->", Token::Arrow);

// keywords
lex_token_test!(_return, "return", Token::Return);
lex_token_test!(inline, "inline", Token::Inline);
lex_token_test!(typedef, "typedef", Token::Typedef);
lex_token_test!(_extern, "extern", Token::Extern);
lex_token_test!(_static, "static", Token::Static);
lex_token_test!(auto, "auto", Token::Auto);
lex_token_test!(register, "register", Token::Register);
lex_token_test!(_const, "const", Token::Const);
lex_token_test!(restrict, "restrict", Token::Restrict);
lex_token_test!(volatile, "volatile", Token::Volatile);
lex_token_test!(void, "void", Token::Void);
lex_token_test!(char, "char", Token::Char);
lex_token_test!(short, "short", Token::Short);
lex_token_test!(int, "int", Token::Int);
lex_token_test!(long, "long", Token::Long);
lex_token_test!(float, "float", Token::Float);
lex_token_test!(double, "double", Token::Double);
lex_token_test!(signed, "signed", Token::Signed);
lex_token_test!(unsigned, "unsigned", Token::Unsigned);

// identifiers
lex_token_test!(ident1, "a", Token::Identifier("a"));
lex_token_test!(ident_begins_with_keyword1, "return_addr", Token::Identifier("return_addr"));
lex_token_test!(ident_begins_with_keyword2, "external", Token::Identifier("external"));

// constants
// TODO: octal, binary int literals
// TODO: string literals
// TODO: floating point literals
lex_token_test!(decimal_int_literal, "425", Token::IntLiteral("425"));
lex_token_test!(hexadecimal_int_literal, "0x425", Token::IntLiteral("0x425"));
