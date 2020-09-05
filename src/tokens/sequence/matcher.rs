use std::mem::Discriminant;
use crate::tokens::Token;
use crate::ast_parser::AstNode;
pub trait TokenSequenceMatcher {
    fn get_matching_sequence(&self) -> Vec<Discriminant<Token>>;
    fn tokenize(&self, tokens: &Vec<Token>) -> AstNode;
}