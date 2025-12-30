pub mod opcodes;
use opcodes::OpCode;

#[derive(Debug, Clone, Copy)]
pub enum StackSlotVariant {
    Int,
    Float
}

pub union StackSlotData {
    pub ival: i64,
    pub fval: f64
}

pub struct StackSlot {
    pub data: StackSlotData,
    pub variant: StackSlotVariant
}

impl Clone for StackSlot {
    fn clone(&self) -> Self {
        let new_union = match self.variant {
            StackSlotVariant::Int => StackSlotData { ival: unsafe { self.data.ival } },
            StackSlotVariant::Float => StackSlotData { fval: unsafe { self.data.fval } }
        };

        return StackSlot { data: new_union, variant: self.variant };
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
                    self.push(StackSlot { variant: StackSlotVariant::Int, data: StackSlotData { ival: unsafe { lhs.data.ival + rhs.data.ival } }});
                }
                Some(OpCode::ISub) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot { variant: StackSlotVariant::Int, data: StackSlotData { ival: unsafe { lhs.data.ival - rhs.data.ival } }});
                }
                Some(OpCode::IMul) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot { variant: StackSlotVariant::Int, data: StackSlotData { ival: unsafe { lhs.data.ival * rhs.data.ival } }});
                }
                Some(OpCode::IDiv) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot { variant: StackSlotVariant::Int, data: StackSlotData { ival: unsafe { lhs.data.ival / rhs.data.ival } }});
                }
                Some(OpCode::IRem) => {
                    self.bc_pos += 1;
                    let rhs = self.pop();
                    let lhs = self.pop();
                    self.push(StackSlot { variant: StackSlotVariant::Int, data: StackSlotData { ival: unsafe { lhs.data.ival % rhs.data.ival } }});
                }
                Some(OpCode::Print) => {
                    self.bc_pos += 1;
                    let val = self.pop();
                    match val.variant {
                        StackSlotVariant::Int => unsafe { println!("{}", val.data.ival); }
                        StackSlotVariant::Float => unsafe { println!("{}", val.data.fval); }
                    }
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