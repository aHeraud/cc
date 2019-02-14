mod primary_expression {
    use crate::lexer::Lexer;
    use ast::*;
    use crate::parser::grammar::*;

    #[test]
    fn primary_expression_ident() {
        let lexer = Lexer::new("test.c".into(), "a");
        let p_exp = PrimaryExpressionParser::new()
            .parse(lexer)
            .unwrap();
        assert_eq!(p_exp, PrimaryExpression::Identifier("a".into()));
    }

    #[test]
    fn constant_int() {
        let lexer = Lexer::new("test.c".into(), "42");
        let p_exp = PrimaryExpressionParser::new()
            .parse(lexer)
            .unwrap();
        assert_eq!(p_exp, PrimaryExpression::Constant(Constant::Integer(Integer::I32(42))));
    }

    // TODO: (expr)
}

mod postfix_expression {
    use crate::lexer::Lexer;
    use crate::parser::grammar::*;
    use ast::*;

    #[test]
    fn array_access_constant_index() {
        let input = Lexer::new("test.c".into(), "numbers[10]");
        let expr = PostfixExpressionParser::new()
            .parse(input)
            .unwrap();
        assert_eq!(expr, vec![
            PostfixExpressionPart::PrimaryExpression(PrimaryExpression::Identifier("numbers".into())),
            // oh god why is this so deeply nested
            PostfixExpressionPart::ArrayAccess(Box::new(vec![
                AssignmentExpression::ConditionalExpression(
                    Box::new(ConditionalExpression::LogicalOrExpression(
                        Box::new(LogicalOrExpression::LogicalAndExpression(
                            Box::new(LogicalAndExpression::OrExpression(
                                Box::new(OrExpression::XorExpression(
                                    Box::new(XorExpression::AndExpression(
                                        Box::new(AndExpression::EqualityExpression(
                                            Box::new(EqualityExpression::RelationalExpression(
                                                Box::new(RelationalExpression::ShiftExpression(
                                                    Box::new(ShiftExpression::AdditiveExpression(
                                                        Box::new(AdditiveExpression::MultiplicativeExpression(
                                                            Box::new(MultiplicativeExpression::CastExpression(
                                                                Box::new(CastExpression::UnaryExpression(
                                                                    Box::new(UnaryExpression::PostfixExpression(
                                                                        vec![PostfixExpressionPart::PrimaryExpression(PrimaryExpression::Constant(Constant::Integer(Integer::I32(10))))]
                                                                    )
                                                                ))
                                                            ))
                                                        ))
                                                    ))
                                                ))
                                            ))
                                        ))
                                    ))
                                ))
                            ))
                        ))
                    ))
                ))
            ]))
        ]);
    }

    #[test]
    fn function_call_empty_argument_expression_list() {
        let input = Lexer::new("test.c".into(), "foo()");
        let expr = PostfixExpressionParser::new()
            .parse(input)
            .unwrap();
        assert_eq!(expr, vec![
            PostfixExpressionPart::PrimaryExpression(PrimaryExpression::Identifier("foo".into())),
            PostfixExpressionPart::ArgumentExpressionList(vec![])
        ]);
    }
}