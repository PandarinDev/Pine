use crate::tokens::Token;
pub struct Tokenizer {
    tokens: Vec<Token>,
    in_string_literal: bool,
    in_numeric_literal: bool,
    buffer: String
}

impl Tokenizer {

    pub fn new() -> Self {
        Tokenizer {
            tokens: vec![],
            in_string_literal: false,
            in_numeric_literal: false,
            buffer: String::new()
        }
    }

    pub fn tokenize(mut self, source: &str) -> Vec<Token> {
        for char in source.chars() {
            // Do not process characters while inside a string literal
            if self.in_string_literal && char != '"' {
                self.buffer.push(char);
                continue;
            }

            // Skip white space characters when not in string literal
            if char.is_whitespace() {
                continue;
            }

            // Otherwise tokenize every character
            match char {
                '(' => self.flush_buffer_and_add(Token::ParenthesisStart),
                ')' => self.flush_buffer_and_add(Token::ParenthesisEnd),
                ';' => self.flush_buffer_and_add(Token::StatementEnd),
                '+' => self.flush_buffer_and_add(Token::Plus),
                '-' => self.flush_buffer_and_add(Token::Minus),
                '"' => self.handle_string_literal(),
                '0'..='9' => {
                    self.in_numeric_literal = true;
                    self.buffer.push(char);
                },
                _ => self.buffer.push(char)
            }
        }
        self.flush_buffer();

        self.tokens
    }

    fn flush_buffer(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        if self.in_numeric_literal {
            self.tokens.push(Token::NumericLiteral(self.buffer.parse::<i32>().unwrap()));
            self.buffer.clear();
            self.in_numeric_literal = false;
            return;
        }
        self.tokens.push(Token::Identification(self.buffer.clone()));
        self.buffer.clear();
    }

    fn flush_buffer_and_add(&mut self, token: Token) {
        self.flush_buffer();
        self.tokens.push(token);
    }

    fn handle_string_literal(&mut self) {
        if !self.in_string_literal {
            self.in_string_literal = true;
            return;
        }
        self.tokens.push(Token::StringLiteral(self.buffer.clone()));
        self.buffer.clear();
        self.in_string_literal = false;
    }

}