pub struct AstExecutor;

use crate::ast_parser::{AstNode, AstValue};

impl AstExecutor {

    pub fn execute(root: &AstNode) {
        Self::collapse_node(root);
    }

    fn collapse_node(node: &AstNode) {
        for child in &node.children {
            Self::collapse_node(child);
        }
        // TODO At the end of collapsion the node should be replaced
        // with the result in the AST instead of printing it to stdout
        Self::execute_collapsed_node(node);
    }
    
    fn execute_collapsed_node(node: &AstNode) {
        match node.value {
            // TODO Come up with a decent way of associating AstValue
            // discriminants with their respective executor implementations
            AstValue::ADDITION => {
                let first_number = match node.children[0].value {
                    AstValue::NUMBER(num) => num,
                    _ => panic!("Invalid state")
                };
                let second_number = match node.children[1].value {
                    AstValue::NUMBER(num) => num,
                    _ => panic!("Invalid state")
                };
                println!("{}", first_number + second_number);
            },
            AstValue::SUBTRACTION => {
                let first_number = match node.children[0].value {
                    AstValue::NUMBER(num) => num,
                    _ => panic!("Invalid state")
                };
                let second_number = match node.children[1].value {
                    AstValue::NUMBER(num) => num,
                    _ => panic!("Invalid state")
                };
                println!("{}", first_number - second_number);
            },
            _ => {}
        }
    }

}