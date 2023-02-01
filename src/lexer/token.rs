

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(i32),
    //String(String),

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

}
