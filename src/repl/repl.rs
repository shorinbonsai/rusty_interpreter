use lexer::lexer::{Lexer};
use lexer::token::{Token};
use std::io::{self, BufRead, Write};


use crate::lexer;

pub fn start() {
    let stdin = io::stdin();
    loop {
        // Stdout needs to be flushed, due to missing newline
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");
        let mut lexer = Lexer::new(line);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}