// use crate::lexer::lexer::*;
use crate::lexer::token::*;

use super::token::Token;



pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Default for Lexer {
    fn default() -> Lexer {
        Lexer {input: String::new(), position: 0, read_position: 0, ch: '\0' }
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer::default();
        l.input = input;
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            }
        }
        self.position = self.read_position;
        self.read_position += 1;

    }

    fn next_token(&mut self) -> Token {
        let tok: Token;
        
        match self.ch {
            '+' => tok = Token::Plus,
            '=' => tok = Token::Assign,
            '(' => tok = Token::Lparen,
            ')' => tok = Token::Rparen,
            '{' => tok = Token::Lbrace,
            '}' => tok = Token::Rbrace,
            ',' => tok = Token::Comma,
            ';' => tok = Token::Semicolon,
            '\0' => tok = Token::EOF,
            _ => tok = Token::Illegal,
        }
        self.read_char();
        return tok;
    }
}


#[test]
fn test_next_token() {
    let input = "=+(){},;";

    let expected = vec![
        Token::Assign,
        Token::Plus,
        Token::Lparen,
        Token::Rparen,
        Token::Lbrace,
        Token::Rbrace,
        Token::Comma,
        Token::Semicolon,
    ];
    let mut l = Lexer::new(input.to_owned());
    for x in expected {
        let tok = l.next_token();
        assert_eq!(tok, x);
    }

}
