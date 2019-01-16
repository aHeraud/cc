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
lex_token_test!(increment, "++", Token::Increment);
lex_token_test!(decrement, "--", Token::Decrement);
lex_token_test!(ampersand, "&", Token::Ampersand);
lex_token_test!(plus, "+", Token::Plus);
lex_token_test!(minus, "-", Token::Minus);
lex_token_test!(tilde, "~", Token::Tilde);
lex_token_test!(exclamation, "!", Token::Exclamation);
lex_token_test!(slash, "/", Token::Slash);
lex_token_test!(modulo, "%", Token::Modulo);
lex_token_test!(caret, "^", Token::Caret);
lex_token_test!(vertical_bar, "|", Token::VerticalBar);
lex_token_test!(colon, ":", Token::Colon);
lex_token_test!(question, "?", Token::Question);
lex_token_test!(shl, "<<", Token::Shl);
lex_token_test!(shr, ">>", Token::Shr);
lex_token_test!(less_than, "<", Token::LessThan);
lex_token_test!(greater_than, ">", Token::GreaterThan);
lex_token_test!(less_than_or_equal_to, "<=", Token::LessThanOrEqualTo);
lex_token_test!(greater_than_or_equal_to, ">=", Token::GreaterThanOrEqualTo);
lex_token_test!(equality, "==", Token::Equality);
lex_token_test!(not_equal, "!=", Token::NotEqual);
lex_token_test!(and_and, "&&", Token::AndAnd);
lex_token_test!(or_or, "||", Token::OrOr);
lex_token_test!(equal, "=", Token::Equal);
lex_token_test!(mult_eq, "*=", Token::MultEq);
lex_token_test!(div_eq, "/=", Token::DivEq);
lex_token_test!(mod_eq, "%=", Token::ModEq);
lex_token_test!(plus_eq, "+=", Token::PlusEq);
lex_token_test!(minus_eq, "-=", Token::MinusEq);
lex_token_test!(shl_eq, "<<=", Token::ShlEq);
lex_token_test!(shr_eq, ">>=", Token::ShrEq);
lex_token_test!(and_eq, "&=", Token::AndEq);
lex_token_test!(or_eq, "|=", Token::OrEq);

// keywords
lex_token_test!(_goto, "goto", Token::Goto);
lex_token_test!(_continue, "continue", Token::Continue);
lex_token_test!(_break, "break", Token::Break);
lex_token_test!(_return, "return", Token::Return);
lex_token_test!(_if, "if", Token::If);
lex_token_test!(_switch, "switch", Token::Switch);
lex_token_test!(case, "case", Token::Case);
lex_token_test!(default, "default", Token::Default);
lex_token_test!(_while, "while", Token::While);
lex_token_test!(_do, "do", Token::Do);
lex_token_test!(_for, "for", Token::For);
lex_token_test!(inline, "inline", Token::Inline);
lex_token_test!(typedef, "typedef", Token::Typedef);
lex_token_test!(_extern, "extern", Token::Extern);
lex_token_test!(_static, "static", Token::Static);
lex_token_test!(auto, "auto", Token::Auto);
lex_token_test!(register, "register", Token::Register);
lex_token_test!(_const, "const", Token::Const);
lex_token_test!(restrict, "restrict", Token::Restrict);
lex_token_test!(volatile, "volatile", Token::Volatile);
lex_token_test!(_struct, "struct", Token::Struct);
lex_token_test!(_union, "union", Token::Union);
lex_token_test!(_enum, "enum", Token::Enum);
lex_token_test!(void, "void", Token::Void);
lex_token_test!(char, "char", Token::Char);
lex_token_test!(short, "short", Token::Short);
lex_token_test!(int, "int", Token::Int);
lex_token_test!(long, "long", Token::Long);
lex_token_test!(float, "float", Token::Float);
lex_token_test!(double, "double", Token::Double);
lex_token_test!(signed, "signed", Token::Signed);
lex_token_test!(unsigned, "unsigned", Token::Unsigned);
lex_token_test!(sizeof, "sizeof", Token::SizeOf);

// identifiers
lex_token_test!(ident1, "a", Token::Identifier("a"));
lex_token_test!(ident_begins_with_keyword1, "return_addr", Token::Identifier("return_addr"));
lex_token_test!(ident_begins_with_keyword2, "external", Token::Identifier("external"));

// string literals
lex_token_test!(string_literal, r#""hello world!""#, Token::StringLiteral{ wide: false, contents: "hello world!"});
lex_token_test!(string_literal_simple_escape, r#""hello world!\r\n""#, Token::StringLiteral{ wide: false, contents: r#"hello world!\r\n"# });
lex_token_test!(string_literal_ucs, r#""\u2699""#, Token::StringLiteral{ wide: false, contents: r#"\u2699"# });
lex_token_test!(string_literal_escaped_quote, r#""\"""#, Token::StringLiteral{ wide: false, contents: r#"\""# });
lex_token_test!(wide_string_literal, r#"L"hello world!""#, Token::StringLiteral{ wide: true, contents: "hello world!" });