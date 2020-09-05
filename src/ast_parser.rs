
use std::mem::Discriminant;
use crate::tokens::{Token, Tokenizer};
use crate::tokens::sequence::TokenSequenceMatcherFacade;
pub struct AstParser;

#[derive(Debug)]
pub struct AstNode {
    pub value: AstValue,
    pub children: Vec<AstNode>
}

#[derive(Debug)]
pub enum AstValue {
    ROOT,
    ADDITION,
    SUBTRACTION,
    NUMBER(i32)
}

impl AstParser {

    pub fn parse(&self, source: &str) -> Result<AstNode, Box<dyn std::error::Error>> {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize(source);
        let mut root_node = AstNode {
            value: AstValue::ROOT,
            children: vec![]
        };
        let sequence_matcher = TokenSequenceMatcherFacade::new();

        let mut token_discriminants = Vec::<Discriminant<Token>>::new();
        let mut token_buffer = Vec::<Token>::new();
        for token in tokens {
            token_discriminants.push(std::mem::discriminant(&token));
            token_buffer.push(token);
            if let Some(matcher) = sequence_matcher.get_matcher(&token_discriminants) {
                let new_node = matcher.tokenize(&token_buffer);
                root_node.children.push(new_node);
                token_buffer.clear();
            }
        }

        Ok(root_node)
    }

}