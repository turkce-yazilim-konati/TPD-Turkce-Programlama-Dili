extern crate libtpd;

#[cfg(test)]
mod tests {
    use crate::libtpd::parser::*;
    use crate::libtpd::types::*;
    use crate::libtpd::syntax::SyntaxParser;
    use crate::libtpd::compiler::value::BramaPrimative;
    use crate::libtpd::compiler::ast::BramaAstType;
    use std::rc::Rc;

    #[warn(unused_macros)]
    macro_rules! test_compare {
        ($name:ident, $text:expr, $result:expr) => {
            #[test]
            fn $name () {
                let mut parser = Parser::new($text);
                match parser.parse() {
                    Err(_) => assert_eq!(true, false),
                    _ => ()
                };

                let syntax = SyntaxParser::new(Box::new(parser.tokens().to_vec()));
                assert_eq!(syntax.parse(), $result);
            }
        };
    }

    test_compare!(assignment_1, "erhan = 2020", Ok(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2020.0))))
    }));

    test_compare!(assignment_2, "erhan = ('erhan' * 2)", Ok(BramaAstType::Assignment {
        variable: Box::new(BramaAstType::Symbol("erhan".to_string())),
        operator: BramaOperatorType::Assign,
        expression: Box::new(BramaAstType::Binary {
            left: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new("erhan".to_string()))))),
            operator: BramaOperatorType::Multiplication, 
            right: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Number(2.0))))
        })
    }));
}