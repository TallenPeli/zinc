use crate::tokenizer::{Token, TokenType};

pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
}

pub enum Node {
    List(Vec<Node>),
    Int(i32),
    Float(f32),
    Binary {
        left: Box<Node>,
        op: Operator,
        right: Box<Node>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    fn parse() {
        let mut program: Vec<Node> = Vec::new();
        program.push(Node::List(Vec::new()));
    }

    fn match_token(&self, expected: TokenType) -> bool {
        matches!(self.peek(0), Some(token) if token.tok_type == expected)
    }

    fn peek(&self, forward: usize) -> Option<&Token> {
        if self.index + forward <= self.tokens.len() {
            self.tokens.get(self.index + forward)
        } else {
            None
        }
    }

    fn consume(&mut self, amount: usize) {
        if self.index + amount <= self.tokens.len() {
            self.index += amount
        }
    }
}
