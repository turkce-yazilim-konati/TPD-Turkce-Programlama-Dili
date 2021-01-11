use std::rc::Rc;

use crate::types::*;
use crate::syntax::{SyntaxParser, SyntaxParserTrait, SyntaxFlag, ExtensionSyntaxParser};
use crate::syntax::binary::AddSubtractParser;
use crate::syntax::func_call::FuncCallParser;
use crate::syntax::unary::UnaryParser;
use crate::syntax::util::update_functions_for_temp_return;
use crate::compiler::ast::BramaAstType;
use crate::compiler::value::BramaPrimative;

pub struct ExpressionParser;
pub struct OrParser;
pub struct AndParser;
pub struct EqualityParser;
pub struct ControlParser;

impl SyntaxParserTrait for ExpressionParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        let mut ast = OrParser::parse(parser)?;

        /* parse for 'object()()' */
        if FuncCallParser::parsable(parser) {
            update_functions_for_temp_return(&mut ast);
            return FuncCallParser::parse_suffix(Box::new(ast), parser);
        }
        
        /* parse for 'object.method()' or 'object.method' */
        else if let Some(_) = parser.match_operator(&[BramaOperatorType::Dot]) {
            let sub_ast = ExpressionParser::parse(parser)?;
            
            let ast = match &sub_ast {
                BramaAstType::Symbol(symbol) => {
                    BramaAstType::Indexer 
                    { 
                        body: Box::new(ast),
                        
                        /* Convert symbol to text */
                        indexer: Box::new(BramaAstType::Primative(Rc::new(BramaPrimative::Text(Rc::new(symbol.to_string()))))) 
                    }
                },
                BramaAstType::FuncCall {
                    func_name_expression,
                    arguments: _,
                    assign_to_temp: _ 
                } => {
                    match &**func_name_expression {
                        BramaAstType::Symbol(_) => {
                            update_functions_for_temp_return(&mut ast);
                            BramaAstType::AccessorFuncCall {
                                source: Box::new(ast),
                                indexer: Box::new(sub_ast),
                                assign_to_temp: false
                            }
                        },
                        _ => return Err(("Function call syntax not valid", 0, 0))
                    }
                },
                 _ => return Err(("Function call syntax not valid", 0, 0))
            };

            return Ok(ast);
        }
        
        /* parse for '["data"]' */
        else if parser.check_operator(&BramaOperatorType::SquareBracketStart) {
            return UnaryParser::parse_indexer(Box::new(ast), parser);
        }

        Ok(ast)
    }
}

impl SyntaxParserTrait for OrParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parse_control::<AndParser>(parser, &[BramaOperatorType::Or])
    }
}

impl SyntaxParserTrait for AndParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parse_control::<EqualityParser>(parser, &[BramaOperatorType::And])
    }
}

impl SyntaxParserTrait for EqualityParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parse_control::<ControlParser>(parser, &[BramaOperatorType::Equal, BramaOperatorType::NotEqual])
    }
}

impl SyntaxParserTrait for ControlParser {
    fn parse(parser: &SyntaxParser) -> AstResult {
        parse_control::<AddSubtractParser>(parser, &[BramaOperatorType::GreaterEqualThan, 
            BramaOperatorType::GreaterThan,
            BramaOperatorType::LessEqualThan, 
            BramaOperatorType::LessThan])
    }
}

pub fn parse_control<T: SyntaxParserTrait>(parser: &SyntaxParser, operators: &[BramaOperatorType]) -> AstResult {
    let mut functions_updated_for_temp = false;
    let mut left_expr = T::parse(parser)?;
    match left_expr {
        BramaAstType::None => return Ok(left_expr),
        _ => ()
    };
    
    loop {
        let index_backup = parser.get_index();
        parser.cleanup_whitespaces();
        if let Some(operator) = parser.match_operator(operators) {
            if !functions_updated_for_temp {
                update_functions_for_temp_return(&mut left_expr);
                functions_updated_for_temp = true;
            }

            parser.cleanup_whitespaces();
            let parser_flags  = parser.flags.get();
            parser.flags.set(parser_flags | SyntaxFlag::IN_EXPRESSION);
            
            let right_expr = T::parse(parser);
            match right_expr {
                Ok(BramaAstType::None) => {
                    parser.set_index(index_backup);
                    return Err(("Right side of expression not found", 0, 0));
                },
                Ok(_) => (),
                Err(_) => return right_expr
            };

            parser.flags.set(parser_flags);
            left_expr = BramaAstType::Control {
                left: Box::new(left_expr),
                operator,
                right: Box::new(right_expr.unwrap())
            };
        }        
        else {
            parser.set_index(index_backup);
            break;
        }
    }

    Ok(left_expr)
}
