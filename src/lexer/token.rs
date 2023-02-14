

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(i32),
    String(String),
    Bool(bool),

    Assign,
    Plus,
    //Minus,
    //Bang,
    //Asterisk,
    //Slash,

    //Lt,
    //Gt,
    //Eq,
    //Not_eq,

    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    If,
    Else,
    Return,

}

impl Token {
    pub fn lookup_ident(ident: String) -> Token {
	match ident.as_str() {
	    "fn" => Token::Function,
	    "let" => Token::Let,
	    "true" => Token::Bool(true),
	    "false" => Token::Bool(false),
	    "if" => Token::If,
	    "else" => Token::Else,
	    "return" => Token::Return,
	    _ => Token::Ident(ident.to_string())
}}}
