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

pub struct VM {
    constants: Vec<StackSlot>,
    evaluated_stack: Vec<StackSlot>,
    bytecode: Vec<u8>,
    bc_pos: usize
}

impl VM {
    pub fn new() -> Self {
        Self{constants: Vec::new(), evaluated_stack: Vec::new(), bytecode: Vec::new(), bc_pos: 0}
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }

    pub fn emit_const(&mut self, slot: StackSlot) {
        self.constants.push(slot);
        let index = self.constants.len() - 1;
        self.emit_byte(OpCode::Push as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
    }

    pub fn push(&mut self, slot: StackSlot) {
        self.evaluated_stack.push(slot);
    }

    pub fn pop(&mut self) -> StackSlot {
        return self.evaluated_stack.pop().unwrap();
    }

    pub fn execute(&mut self) {
        while self.bc_pos < self.bytecode.len() {
            match OpCode::from_u8(self.bytecode[self.bc_pos]) {
                Some(OpCode::Push) => {
                    self.bc_pos += 1;
                    let index = self.get_index();
                    self.push(self.constants[index].clone());
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
        index += (self.bytecode[self.bc_pos] as usize) << 16;
        index += (self.bytecode[self.bc_pos + 1] as usize) << 8;
        index += self.bytecode[self.bc_pos + 2] as usize;
        self.bc_pos += 3;
        return index;
    }
}