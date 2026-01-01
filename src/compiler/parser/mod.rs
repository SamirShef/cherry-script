use crate::{
    compiler::{
        lexer::token::Token,
        location::Location
    },
    vm::{
        VM,
        chunk::Chunk,
        opcodes::OpCode,
        stack_slot::StackSlot
    }
};

pub struct Parser {
    tokens: Vec<Option<Token>>,
    pos: usize,
    globals: Vec<String>,
    pub vm: VM
}

impl Parser {
    pub fn new(tokens: Vec<Option<Token>>) -> Self {
        Self { tokens: tokens, pos: 0, globals: Vec::new(), vm: VM::new(vec![Chunk::new()]) }
    }

    pub fn generate(&mut self) {
        while self.pos < self.tokens.len() {
            match self.advance() {
                Some(Token::Var(loc)) | Some(Token::Const(loc)) => self.generate_var_def(loc),
                Some(Token::Print(_)) => self.generate_print(),
                _ => {}
            }
        }
    }

    fn generate_var_def(&mut self, loc: Location) {
        let name = match self.advance() {
            Some(Token::Id(name, _)) => name,
            _ => panic!("Error: Expected identifier")
        };
        
        match self.peek(0) {
            Some(Token::Assign(_)) => {
                self.advance();
                Some(self.generate_expr());
                match self.peek(0) {
                    Some(Token::Semi(_)) => { self.advance(); }
                    _ => panic!("Error: Expected `;`: {:?}", self.peek(0))
                }
            }
            Some(Token::Semi(_)) => { self.advance(); }
            _ => panic!("Error: Unexpected symbol: {:?}", self.peek(0))
        }
        
        if self.vm.chunk_index == 0 {
            self.generate_global_var(name);
        }
        else {
            self.generate_local_var(name);
        }
    }

    fn generate_global_var(&mut self, name: String) {
        let index = self.vm.create_global();
        self.globals.push(name);
        self.vm.store_global(index);
    }

    fn generate_local_var(&mut self, name: String) {
        todo!()
    }

    fn generate_print(&mut self) {
        self.generate_expr();
        self.vm.chunks[self.vm.chunk_index].emit_byte(OpCode::Print as u8);
        match self.peek(0) {
            Some(Token::Semi(_)) => { self.advance(); }
            _ => panic!("Error: Expected `;`: {:?}", self.peek(0))
        }
    }

    fn generate_expr(&mut self) {
        self.generate_additive_expr();
    }

    fn generate_additive_expr(&mut self) {
        self.generate_multiplicative_expr();
        while let tok = self.peek(0) {
            match tok {
                Some(Token::Plus(_)) => {
                    self.advance();
                    self.generate_multiplicative_expr();
                    self.vm.chunks[self.vm.chunk_index].emit_byte(OpCode::Add as u8);
                },
                Some(Token::Minus(_)) => {
                    self.advance();
                    self.generate_multiplicative_expr();
                    self.vm.chunks[self.vm.chunk_index].emit_byte(OpCode::Sub as u8);
                },
                _ => break
            }
        }
    }

    fn generate_multiplicative_expr(&mut self) {
        self.generate_primary_expr();
        while let tok = self.peek(0) {
            match tok {
                Some(Token::Star(_)) => {
                    self.advance();
                    self.generate_primary_expr();
                    self.vm.chunks[self.vm.chunk_index].emit_byte(OpCode::Mul as u8);
                },
                Some(Token::Slash(_)) => {
                    self.advance();
                    self.generate_primary_expr();
                    self.vm.chunks[self.vm.chunk_index].emit_byte(OpCode::Div as u8);
                },
                Some(Token::Percent(_)) => {
                    self.advance();
                    self.generate_primary_expr();
                    self.vm.chunks[self.vm.chunk_index].emit_byte(OpCode::Rem as u8);
                },
                _ => break
            }
        }
    }

    fn generate_primary_expr(&mut self) {
        let tok = self.advance();
        match tok {
            Some(Token::Id(name, _)) => {
                let var_index = self.globals.iter().position(|s| **s == name);
                match var_index {
                    Some(index) => { self.vm.load_global(index); },
                    None => panic!("Error: Variable does not defined")
                }
            }
            Some(Token::Int(val, _)) => {
                self.vm.chunks[self.vm.chunk_index].emit_const(StackSlot::Int(val));
            },
            Some(Token::Float(val, _)) => {
                self.vm.chunks[self.vm.chunk_index].emit_const(StackSlot::Float(val));
            },
            _ => panic!("Error: Expected expression")
        }
    }

    fn peek(&mut self, rpos: i32) -> Option<Token> {
        if self.pos as i128 + rpos as i128 >= self.tokens.len() as i128 {
            panic!("Index passed to parser out of bounds: {} + {} / {}", self.pos, rpos, self.tokens.len());
        }
        return self.tokens[(self.pos as i128 + rpos as i128) as usize].clone();
    }

    fn advance(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let tok = self.peek(0);
            self.pos += 1;
            return tok;
        }
        return None;
    }
}