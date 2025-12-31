use super::stack_slot::StackSlot;
use super::OpCode;

pub struct Chunk {
    pub constants: Vec<StackSlot>,
    pub bytecode: Vec<u8>,
    pub locals: Vec<Option<StackSlot>>
}

impl Chunk {
    pub fn new() -> Self {
        Self { constants: Vec::new(), bytecode: Vec::new(), locals: Vec::new() }
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

    pub fn emit_call(&mut self, index: usize) -> usize {
        let first_instruction = self.emit_byte(OpCode::Call as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
        return first_instruction;
    }

    pub fn create_local(&mut self) -> usize {
        self.locals.push(None);
        return self.locals.len() - 1;
    }

    pub fn store_local(&mut self, index: usize) {
        self.emit_byte(OpCode::StoreLoc as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
    }

    pub fn load_local(&mut self, index: usize) {
        self.emit_byte(OpCode::LoadLoc as u8);
        self.emit_byte(((index >> 16) & 0xFF) as u8);
        self.emit_byte(((index >> 8) & 0xFF) as u8);
        self.emit_byte((index & 0xFF) as u8);
    }

    pub fn get_local(&mut self, index: usize) -> StackSlot {
        return self.locals[index].clone().unwrap();
    }
}