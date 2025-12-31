mod vm;

use vm::{Chunk, StackSlot, VM};
use vm::opcodes::OpCode;

fn main() {
    let mut chunks = Vec::new();
    let mut chunk = Chunk::new();
    chunk.emit_const(StackSlot::Int(3));
    chunk.emit_const(StackSlot::Int(2));
    chunk.emit_byte(OpCode::IAdd as u8);
    chunk.emit_byte(OpCode::Print as u8);
    
    chunks.push(chunk);
    
    let mut vm = VM::new(chunks);

    let glob_index = vm.create_global();
    vm.store_global(glob_index, StackSlot::Int(67));
    vm.load_global(glob_index);
    vm.chunks[vm.chunk_index].emit_byte(OpCode::Print as u8);

    vm.chunks[vm.chunk_index].emit_const(StackSlot::Int(1));    // inverted condition
    vm.chunks[vm.chunk_index].emit_jmp_if(36);   // jmp to the body end

    // body (then)
    vm.load_global(glob_index);
    vm.chunks[vm.chunk_index].emit_byte(OpCode::Print as u8);
    vm.chunks[vm.chunk_index].emit_jmp(u8::MAX as u32);   // jmp is far from here (to stop the execution)

    // body end
    vm.chunks[vm.chunk_index].emit_const(StackSlot::Int(1488));
    vm.chunks[vm.chunk_index].emit_byte(OpCode::Print as u8);
    
    vm.execute();
}