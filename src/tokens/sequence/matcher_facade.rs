use std::collections::HashMap;
use std::mem::Discriminant;
use crate::tokens::Token;
use crate::tokens::sequence::{
    TokenSequenceMatcher,
    AdditionSequenceMatcher,
    SubtractionSequenceMatcher
};

pub type MemoizedSequenceMatchers = HashMap<Vec<Discriminant<Token>>, Box<dyn TokenSequenceMatcher>>;

pub struct TokenSequenceMatcherFacade {
    memoized_sequence_matchers: MemoizedSequenceMatchers
}

impl TokenSequenceMatcherFacade {

    pub fn new() -> Self {
        let matcher_instances: Vec<Box<dyn TokenSequenceMatcher>> = vec![
            Box::new(AdditionSequenceMatcher),
            Box::new(SubtractionSequenceMatcher)
        ];
        let mut memoized_sequence_matchers = MemoizedSequenceMatchers::new();
        for matcher in matcher_instances {
            memoized_sequence_matchers.insert(matcher.get_matching_sequence(), matcher);
        }

        TokenSequenceMatcherFacade { memoized_sequence_matchers }
    }

    pub fn get_matcher(&self, tokens: &Vec<Discriminant<Token>>) -> Option<&Box<dyn TokenSequenceMatcher>> {
        for entry in &self.memoized_sequence_matchers {
            if entry.0 == tokens {
                return Some(entry.1);
            }
        }
        None
    }

}