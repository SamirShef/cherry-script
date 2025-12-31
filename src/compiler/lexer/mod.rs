pub mod token;
use token::Token;

use crate::compiler::location::Location;

pub struct Lexer {
    src: String,
    pos: usize,
    line: u64,
    col: u64
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self { src: src, pos: 0, line: 1, col: 1 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while self.pos < self.src.len() && (self.peek(0) == ' ' || self.peek(0) == '\n') {
            self.advance();
        }

        let c = self.src.chars().nth(self.pos);
        match c {
            Some(c) if c.is_digit(10)     => Some(self.tokenize_num_lit()),
            Some(c) if c.is_alphabetic()  => Some(self.tokenize_id()),
            Some(c) if c == '"'           => Some(self.tokenize_str_lit()),
            Some(c) if c == '\''          => Some(self.tokenize_char_lit()),
            Some(_c)                      => self.tokenize_op(),
            None => None
        }
    }

    fn tokenize_id(&mut self) -> Token {
        let tmp_l = self.line;
        let tmp_c = self.col;
        let mut val = String::new();
        while self.pos < self.src.len() && (self.peek(0).is_alphanumeric() || self.peek(0) == '_') {
            val.push(self.advance());
        }

        match val.as_str() {
            "var" => Token::Var(Location { line: tmp_l, col: tmp_c }),
            _ => Token::Id(val, Location { line: tmp_l, col: tmp_c })
        }
    }

    fn tokenize_num_lit(&mut self) -> Token {
        let tmp_l = self.line;
        let tmp_c = self.col;
        let mut val = String::new();
        let mut has_dot = false;
        while self.pos < self.src.len() && (self.peek(0).is_digit(10) || self.peek(0) == '_') {
            if self.peek(0) == '.' {
                if has_dot {
                    panic!("Twice dot");
                }
                has_dot = true;
            }
            if self.peek(0) == '_' {
                continue;
            }
            val.push(self.advance());
        }

        if (has_dot) {
            let num = val.parse::<f64>();
            match num {
                Ok(num) => return Token::Float(num, Location { line: tmp_l, col: tmp_c }),
                Err(err) => panic!("Error: {}", err)
            }
        }
        let num = val.parse::<i64>();
        match num {
            Ok(num) => return Token::Int(num, Location { line: tmp_l, col: tmp_c }),
            Err(err) => panic!("Error: {}", err)
        }
    }

    fn tokenize_char_lit(&mut self) -> Token {
        let tmp_l = self.line;
        let tmp_c = self.col;
        let mut val = String::new();
        self.advance();
        while self.pos < self.src.len() && self.peek(0) != '\'' {
            if val.len() == 1 {
                panic!("To many symbols in character literal");
            }
            val.push(self.advance());
        }
        if val.len() == 0 {
            panic!("The character constant must have a length of 1");
        }
        self.advance();

        return Token::Char(val.chars().nth(0).unwrap(), Location { line: tmp_l, col: tmp_c });
    }

    fn tokenize_str_lit(&mut self) -> Token {
        let tmp_l = self.line;
        let tmp_c = self.col;
        let mut val = String::new();
        self.advance();
        while self.pos < self.src.len() && self.peek(0) != '"' {
            val.push(self.advance());
        }
        self.advance();

        return Token::Str(val, Location { line: tmp_l, col: tmp_c });
    }

    fn tokenize_op(&mut self) -> Option<Token> {
        let tmp_l = self.line;
        let tmp_c = self.col;
        let location = Location { line: tmp_l, col: tmp_c };
        let c = self.advance();
        match c {
            '=' if self.pos < self.src.len() &&
                self.peek(0) == '='                 => { self.advance(); Some(Token::Eq(location)) },
            '='                                     => Some(Token::Assign(location)),

            '>' if self.pos < self.src.len() &&
                self.peek(0) == '='                 => { self.advance(); Some(Token::GtEq(location))},
            '>'                                     => Some(Token::Gt(location)),

            '<' if self.pos < self.src.len() &&
                self.peek(0) == '='                 => { self.advance(); Some(Token::LtEq(location)) },
            '<'                                     => Some(Token::Lt(location)),

            '!' if self.pos < self.src.len() &&
                self.peek(0) == '='                 => { self.advance(); Some(Token::NotEq(location)) },
            '!'                                     => Some(Token::Not(location)),

            '&' if self.pos < self.src.len() &&
                self.peek(0) == '&'                 => { self.advance(); Some(Token::LogicalAnd(location)) },
            '&'                                     => Some(Token::And(location)),

            '|' if self.pos < self.src.len() &&
                self.peek(0) == '|'                 => { self.advance(); Some(Token::LogicalOr(location)) },
            '|'                                     => Some(Token::Or(Location { line: tmp_l, col: tmp_c })),
            
            '+'                                     => Some(Token::Plus(location)),
            '-'                                     => Some(Token::Minus(location)),
            '*'                                     => Some(Token::Star(location)),
            '/'                                     => Some(Token::Slash(location)),
            '%'                                     => Some(Token::Percent(location)),
            ';'                                     => Some(Token::Semi(location)),
            ':'                                     => Some(Token::Colon(location)),
            '.'                                     => Some(Token::Dot(location)),
            ','                                     => Some(Token::Comma(location)),
            _                                       => None
        }
    }

    fn peek(&mut self, rpos: usize) -> char {
        if self.pos + rpos >= self.src.len() {
            panic!("Index passed to lexer out of bounds: {} + {} / {}", self.pos, rpos, self.src.len());
        }
        return self.src.chars().nth(self.pos + rpos).unwrap();
    }

    fn advance(&mut self) -> char {
        let c = self.peek(0);
        self.pos += 1;
        match c {
            '\n' => {
                self.col = 1;
                self.line += 1;
            },
            _ => self.col += 1
        }
        return c;
    }
}