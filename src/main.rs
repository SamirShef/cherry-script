mod vm;
use vm::{
    VM,
    opcodes::OpCode,
    chunk::Chunk,
    stack_slot::StackSlot
};


/*
fun test_func() {
    var a = 10;
    a += 10;
    ret a;
}

print(test_func()); // output: Int(20)
 */
fn main() {
    let mut chunks = Vec::new();
    let mut chunk = Chunk::new();
    chunk.emit_call(1);
    chunk.emit_byte(OpCode::Print as u8);
    
    chunks.push(chunk);
    
    let mut vm = VM::new(chunks);

    let mut test_func = Chunk::new();
    test_func.emit_const(StackSlot::Int(10));
    test_func.create_local();
    test_func.store_local(0);

    test_func.load_local(0);
    test_func.emit_const(StackSlot::Int(10));
    test_func.emit_byte(OpCode::IAdd as u8);
    test_func.store_local(0);
    
    test_func.load_local(0);
    test_func.emit_byte(OpCode::Ret as u8);

    vm.add_chunk(test_func);
    
    vm.execute();
}