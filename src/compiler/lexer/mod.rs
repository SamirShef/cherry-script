pub mod token;
use token::Token;

pub struct Lexer {
    src: String,
    pos: u128,
    line: u64,
    col: u64
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self { src: src, pos: 0, line: 1, col: 1 }
    }

    pub fn next_token() -> Token {
        todo!()
    }

    fn tokenize_id() -> Token {
        todo!()
    }

    fn tokenize_num_lit() -> Token {
        todo!()
    }

    fn tokenize_char_lit() -> Token {
        todo!()
    }

    fn tokenize_str_lit() -> Token {
        todo!()
    }

    fn tokenize_op() -> Token {
        todo!()
    }
}