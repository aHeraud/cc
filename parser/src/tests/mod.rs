use std::ffi::OsString;

use crate::lexer::Lexer;
use ast::*;
use crate::parser::grammar::*;

mod declarations;
mod expressions;

#[test]
fn hello_world() {
    let source = r#"
    int main(int argc, char argv[]);

    int main(int argc, char argv[]) {
        printf("hello, world!");
        return 0;
    }"#;
    let input = Lexer::new("test.c".into(), source);
    let ast = TranslationUnitParser::new()
        .parse(input)
        .unwrap();
}