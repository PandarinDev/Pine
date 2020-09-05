#[derive(Debug, PartialEq)]
pub enum Token {
    Identification(String),
    StringLiteral(String),
    NumericLiteral(i32),
    ParenthesisStart,
    ParenthesisEnd,
    StatementEnd,
    Plus,
    Minus
}