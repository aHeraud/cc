#[macro_use] extern crate lalrpop_util;

extern crate ast;
extern crate lexer;
extern crate types;

lalrpop_mod!(grammar);

pub use self::grammar::TranslationUnitParser as Parser;
pub use self::grammar::DeclarationParser;