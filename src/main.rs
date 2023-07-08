mod lexer;
mod repl;
use anyhow::Result;
use crate::repl::repl::start;

fn main() -> Result<()>{
    start();
    println!("Hello, world!");
    return Ok(());
}
