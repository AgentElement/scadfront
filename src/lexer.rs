use std::io::BufReader;

use crate::token;

pub struct Lexer<R> {
    buf: BufReader<R>,
    pos: u64,
}

impl<R> Lexer<R> {
    fn get() -> Option<token::Token> {
        None
    }

    fn peek(k: u64) -> Option<token::Token> {
        None
    }
}
