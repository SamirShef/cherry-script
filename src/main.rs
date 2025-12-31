mod vm;
use vm::{
    VM,
    opcodes::OpCode,
    chunk::Chunk,
    stack_slot::StackSlot
};

fn main() {
    let mut chunks = Vec::new();
    let mut chunk = Chunk::new();
    chunk.emit_const(StackSlot::Int(3));
    chunk.emit_const(StackSlot::Int(2));
    chunk.emit_byte(OpCode::IAdd as u8);
    chunk.emit_byte(OpCode::Print as u8);

    chunk.emit_byte(OpCode::Call as u8);
    chunk.emit_byte(0);
    chunk.emit_byte(0);
    chunk.emit_byte(1);

    chunk.emit_byte(OpCode::Print as u8);
    
    chunks.push(chunk);
    
    let mut vm = VM::new(chunks);

    let mut test_func = Chunk::new();
    test_func.emit_const(StackSlot::Int(67));
    test_func.emit_byte(OpCode::Ret as u8);

    vm.add_chunk(test_func);
    
    vm.execute();
}