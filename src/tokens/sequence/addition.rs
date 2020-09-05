
use std::mem::Discriminant;
use crate::tokens::Token;
use crate::ast_parser::{AstNode, AstValue};
use crate::tokens::sequence::TokenSequenceMatcher;
pub struct AdditionSequenceMatcher;

impl TokenSequenceMatcher for AdditionSequenceMatcher {

    fn get_matching_sequence(&self) -> Vec<Discriminant<Token>> {
        vec![
            std::mem::discriminant(&Token::NumericLiteral(1)),
            std::mem::discriminant(&Token::Plus),
            std::mem::discriminant(&Token::NumericLiteral(1))
        ]
    }

    fn tokenize(&self, tokens: &Vec<Token>) -> AstNode {
        fn get_number(tokens: &Vec<Token>, i: usize) -> i32 {
            match tokens[i] {
                Token::NumericLiteral(num) => num,
                _ => panic!("Token does not contain a numeric literal.")
            }
        }

        AstNode {
            value: AstValue::ADDITION,
            children: vec![
                AstNode {
                    value: AstValue::NUMBER(get_number(tokens, 0)),
                    children: vec![]
                },
                AstNode {
                    value: AstValue::NUMBER(get_number(tokens, 2)),
                    children: vec![]
                }
            ]
        }
    }

}