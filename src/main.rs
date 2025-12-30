mod vm;

use crate::vm::{StackSlot, VM};
use vm::opcodes::OpCode;

fn main() {
    let mut vm = VM::new();
    vm.emit_const(StackSlot::Float(3 as f64));
    vm.emit_const(StackSlot::Float(2 as f64));
    vm.emit_byte(OpCode::FRem as u8);
    vm.emit_byte(OpCode::Print as u8);

    vm.execute();
}