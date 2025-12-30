mod vm;

use crate::vm::{StackSlot, VM};
use vm::opcodes::OpCode;

fn main() {
    let mut vm = VM::new();
    vm.emit_const(StackSlot::Float(3 as f64));
    vm.emit_const(StackSlot::Float(2 as f64));
    vm.emit_byte(OpCode::FRem as u8);
    vm.emit_byte(OpCode::Print as u8);
    let glob_index = vm.create_global();
    vm.store_global(glob_index, StackSlot::Int(3));
    vm.load_global(glob_index);
    vm.emit_byte(OpCode::Print as u8);

    vm.execute();
}