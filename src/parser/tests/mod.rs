use std::ffi::OsString;

use crate::lexer::Lexer;
use crate::parser::ast::*;
use crate::parser::grammar::*;

mod declarations;
mod expressions;

#[test]
fn hello_world() {
    let source = "
    int main();
    int main(void) {
        return 0;
    }";
    let input = Lexer::new("test.c".into(), source);
    let ast = TranslationUnitParser::new()
        .parse(input)
        .unwrap();
}
