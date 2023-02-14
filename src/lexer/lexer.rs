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

    fn read_identifier(&mut self) -> String {
	let position = self.position;
	while Lexer::is_letter(self.ch){
	    self.read_char();
	}
	return self.input[position..self.position].to_string();
    }

    fn is_letter(character: char) -> bool {
	return character.is_ascii_alphanumeric() || character == '_';
    }

    fn next_token(&mut self) -> Token {
        let tok: Token;
	self.skip_whitespace();
        
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
            _ => {
		if Lexer::is_letter(self.ch) {
		    let identifier = self.read_identifier();
		    let tok = Token::lookup_ident(identifier); 
		    return tok;
		} else {
		    tok = Token::Illegal;
		}
	    } 
        }
        self.read_char();
        return tok;
    }

    fn skip_whitespace(&mut self) {
	while self.ch.is_whitespace() {
	    self.read_char();
	}
    }
}


#[test]
fn test_next_token() {
    let input = r#"let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);"#;


    let expected = vec![
        Token::Let,
		Token::Ident(String::from("three")),
		Token::Assign,
		Token::Int(3),
		Token::Semicolon,
		Token::Let,
		Token::Ident(String::from("seven")),
		Token::Assign,
		Token::Int(7),
		Token::Semicolon,
		Token::Let,
		Token::Ident(String::from("add")),
		Token::Assign,
		Token::Function,
		Token::Lparen,
		Token::Ident(String::from("x")),
		Token::Comma,
		Token::Ident(String::from("y")),
		Token::Rparen,
		Token::Assign,
		Token::Lbrace,
		Token::Ident(String::from("x")),
		Token::Plus,
		Token::Ident(String::from("y")),
		Token::Semicolon,
		Token::Rbrace,
		Token::Semicolon,
		Token::Let,
		Token::Ident(String::from("result")),
		Token::Assign,
		Token::Ident(String::from("add")),
		Token::Lparen,
		Token::Ident(String::from("three")),
		Token::Comma,
		Token::Ident(String::from("seven")),
		Token::Rparen,
		Token::Semicolon,
		Token::EOF,
    ];
    let mut l = Lexer::new(input.to_owned());
    for x in expected {
        let tok = l.next_token();
        assert_eq!(tok, x);
    }

}
