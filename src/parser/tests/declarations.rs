use std::ffi::OsString;

use crate::parser::*;
use crate::parser::ast::*;
use crate::lexer::*;



#[test]
fn named_parameter_type_list() {
    let source = "int a, int b, float c";
    let lexer = Lexer::new(OsString::from("test.c") , source);
    let ast = grammar::ParameterTypeListParser::new()
        .parse(lexer)
        .unwrap();
    assert_eq!(ast, ParameterTypeList {
        parameter_list: vec![
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int)],
                declarator: ParameterDeclarator::Declarator(Box::new(Declarator { direct_declarator: vec![DirectDeclaratorPart::Identifier(String::from("a"))] }))
            },
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int)],
                declarator: ParameterDeclarator::Declarator(Box::new(Declarator { direct_declarator: vec![DirectDeclaratorPart::Identifier(String::from("b"))] }))
            },
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Float)],
                declarator: ParameterDeclarator::Declarator(Box::new(Declarator { direct_declarator: vec![DirectDeclaratorPart::Identifier(String::from("c"))] }))
            }
        ],
        variadic: false
    });
}

#[test]
fn unnamed_parameter_type_list() {
    let source = "int, int, float";
    let lexer = Lexer::new(OsString::from("test.c"), source);
    let ast = grammar::ParameterTypeListParser::new()
        .parse(lexer)
        .unwrap();
    assert_eq!(ast, ParameterTypeList {
        parameter_list: vec![
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int)],
                declarator: ParameterDeclarator::AbstractDeclarator(None)
            },
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int)],
                declarator: ParameterDeclarator::AbstractDeclarator(None)
            },
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Float)],
                declarator: ParameterDeclarator::AbstractDeclarator(None)
            }
        ],
        variadic: false
    });
}


#[test]
fn abstract_direct_declarator_vla() {
    let source = "[*]";
    let lexer = Lexer::new(OsString::from("test.c") , source);
    let ast = grammar::DirectAbstractDeclaratorParser::new()
        .parse(lexer)
        .unwrap();
    assert_eq!(ast, vec![DirectAbstractDeclaratorPart::VLA]);
}

#[test]
fn abstract_direct_declarator_parameter_type_list() {
    let source = "(int, int, float)";
    let lexer = Lexer::new(OsString::from("test.c") , source);
    let ast = grammar::DirectAbstractDeclaratorParser::new()
        .parse(lexer)
        .unwrap();
    assert_eq!(ast, vec![DirectAbstractDeclaratorPart::ParameterTypeList(Box::new(
        ParameterTypeList {
        parameter_list: vec![
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int)],
                declarator: ParameterDeclarator::AbstractDeclarator(None)
            },
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Int)],
                declarator: ParameterDeclarator::AbstractDeclarator(None)
            },
            ParameterDeclaration {
                declaration_specifier_list: vec![DeclarationSpecifier::TypeSpecifier(TypeSpecifier::Float)],
                declarator: ParameterDeclarator::AbstractDeclarator(None)
            }
        ],
        variadic: false
    }))]);
}
