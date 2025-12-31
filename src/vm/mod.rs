pub mod opcodes;
use opcodes::OpCode;

#[derive(Debug, Clone)]
pub enum StackSlot {
    Int(i64),
    Float(f64)
}

impl StackSlot {
    fn as_i64(&self) -> Option<i64> {
        match self {
            StackSlot::Int(val) => Some(*val),
            _ => None
        }
    }

    fn as_f64(&self) -> Option<f64> {
        match self {
            StackSlot::Float(val) => Some(*val),
            _ => None
        }
    }
}

pub struct Chunk {
    constants: Vec<StackSlot>,
    bytecode: Vec<u8>
}

impl Chunk {
    pub fn new() -> Self {
        Self { constants: Vec::new(), bytecode: Vec::new() }
    }
    
    pub fn emit_byte(&mut self, byte: u8) -> usize {
        self.bytecode.push(byte);
        return self.bytecode.len() - 1;
    }

    pub fn emit_const(&mut self, slot: StackSlot) -> usize {
        self.constants.push(slot);
        let index = self.constants.len() - 1;
        let first_instruction = self.emit_byte(OpCode::Push as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
        return first_instruction;
    }

    pub fn emit_jmp(&mut self, index: u32) -> usize {
        let first_instruction = self.emit_byte(OpCode::Jmp as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
        return first_instruction;
    }

    pub fn emit_jmp_if(&mut self, index: u32) -> usize {
        let first_instruction = self.emit_byte(OpCode::JmpIf as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
        return first_instruction;
    }
}

pub struct VM {
    evaluated_stack: Vec<StackSlot>,
    pub chunks: Vec<Chunk>,
    pub chunk_index: usize,
    bc_pos: usize,
    globals: Vec<Option<StackSlot>>
}

impl VM {
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self { evaluated_stack: Vec::new(), chunks: chunks, chunk_index: 0, bc_pos: 0, globals: Vec::new() }
    }

    pub fn push(&mut self, slot: StackSlot) {
        self.evaluated_stack.push(slot);
    }

    pub fn pop(&mut self) -> StackSlot {
        return self.evaluated_stack.pop().unwrap();
    }

    pub fn create_global(&mut self) -> usize {
        self.globals.push(None);
        return self.globals.len() - 1;
    }

    pub fn store_global(&mut self, index: usize, slot: StackSlot) {
        self.push(slot);
        self.chunks[self.chunk_index].emit_byte(OpCode::StoreGlob as u8);
        self.chunks[self.chunk_index].emit_byte(((index >> 16) & 0xFF) as u8);
        self.chunks[self.chunk_index].emit_byte(((index >> 8) & 0xFF) as u8);
        self.chunks[self.chunk_index].emit_byte((index & 0xFF) as u8);
    }

    pub fn load_global(&mut self, index: usize) -> usize {
        let first_instruction = self.chunks[self.chunk_index].emit_byte(OpCode::LoadGlob as u8);
        self.chunks[self.chunk_index].emit_byte(((index >> 16) & 0xFF) as u8);
        self.chunks[self.chunk_index].emit_byte(((index >> 8) & 0xFF) as u8);
        self.chunks[self.chunk_index].emit_byte((index & 0xFF) as u8);
        return first_instruction;
    }
    
    pub fn get_global(&mut self) -> StackSlot {
        let index = self.get_index();
        return self.globals[index].clone().unwrap();
    }

    pub fn execute(&mut self) {
        while self.bc_pos < self.chunks[self.chunk_index].bytecode.len() {
            match OpCode::from_u8(self.chunks[self.chunk_index].bytecode[self.bc_pos]) {
                Some(OpCode::Push) => {
                    self.bc_pos += 1;
                    let index = self.get_index();
                    self.push(self.chunks[self.chunk_index].constants[index].clone());
                }
                Some(OpCode::IAdd) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Int(lhs.as_i64().unwrap() + rhs.as_i64().unwrap()));
                }
                Some(OpCode::ISub) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Int(lhs.as_i64().unwrap() - rhs.as_i64().unwrap()));
                }
                Some(OpCode::IMul) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Int(lhs.as_i64().unwrap() * rhs.as_i64().unwrap()));
                }
                Some(OpCode::IDiv) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Int(lhs.as_i64().unwrap() / rhs.as_i64().unwrap()));
                }
                Some(OpCode::IRem) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Int(lhs.as_i64().unwrap() % rhs.as_i64().unwrap()));
                }
                Some(OpCode::FAdd) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Float(lhs.as_f64().unwrap() + rhs.as_f64().unwrap()));
                }
                Some(OpCode::FSub) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Float(lhs.as_f64().unwrap() - rhs.as_f64().unwrap()));
                }
                Some(OpCode::FMul) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Float(lhs.as_f64().unwrap() * rhs.as_f64().unwrap()));
                }
                Some(OpCode::FDiv) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Float(lhs.as_f64().unwrap() / rhs.as_f64().unwrap()));
                }
                Some(OpCode::FRem) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot::Float(lhs.as_f64().unwrap() % rhs.as_f64().unwrap()));
                }
                Some(OpCode::StoreGlob) => {
                    self.bc_pos += 1;
                    let val = self.pop();
                    let index = self.get_index();
                    self.globals[index] = Some(val);
                }
                Some(OpCode::LoadGlob) => {
                    self.bc_pos += 1;
                    let slot = self.get_global();
                    self.push(slot);
                }
                Some(OpCode::Jmp) => {
                    self.bc_pos += 1;
                    self.bc_pos = self.get_index();
                }
                Some(OpCode::JmpIf) => {
                    self.bc_pos += 1;
                    let cond = self.pop();
                    let index = self.get_index();
                    if let Some(val) = cond.as_i64() && val == 1 {
                        self.bc_pos = index;
                    }
                }
                Some(OpCode::Print) => {
                    self.bc_pos += 1;
                    let val = self.pop();
                    println!("{:?}", val)
                }
                _ => { break; }
            }
        }
    }

    pub fn get_index(&mut self) -> usize {
        let mut index = 0;
        index += (self.chunks[self.chunk_index].bytecode[self.bc_pos] as usize) << 16;
        index += (self.chunks[self.chunk_index].bytecode[self.bc_pos + 1] as usize) << 8;
        index += self.chunks[self.chunk_index].bytecode[self.bc_pos + 2] as usize;
        self.bc_pos += 3;
        return index;
    }
}