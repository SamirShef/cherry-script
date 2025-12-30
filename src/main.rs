mod vm;

use crate::vm::{StackSlot, VM};
use vm::opcodes::OpCode;

fn main() {
    let mut vm = VM::new();
    vm.emit_const(StackSlot{data: vm::StackSlotData { ival: 3 }, variant: vm::StackSlotVariant::Int});
    vm.emit_const(StackSlot{data: vm::StackSlotData { ival: 2 }, variant: vm::StackSlotVariant::Int});
    vm.emit_byte(OpCode::IAdd as u8);
    vm.emit_byte(OpCode::Print as u8);

    vm.execute();
}