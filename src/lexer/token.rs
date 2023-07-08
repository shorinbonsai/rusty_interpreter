#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(String),
    String(String),
    Bool(bool),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    Not_eq,
    Lt_eq,
    Gt_eq,

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
            "fn" => return Token::Function,
            "let" => return Token::Let,
            "true" => return Token::Bool(true),
            "false" => return Token::Bool(false),
            "if" => return Token::If,
            "else" => return Token::Else,
            "return" => return Token::Return,
            _ => return Token::Ident(ident.to_string()),
        }
    }
}
 