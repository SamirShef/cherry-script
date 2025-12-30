pub enum OpCode {
    Push = 0,
    Pop = 1,
    IAdd = 2,
    ISub = 3,
    IMul = 4,
    IDiv = 5,
    IRem = 6,
    Print = 7,
}

impl OpCode {
    pub fn from_u8(val: u8) -> Option<OpCode> {
        match val {
            0 => Some(OpCode::Push),
            1 => Some(OpCode::Pop),
            2 => Some(OpCode::IAdd),
            3 => Some(OpCode::ISub),
            4 => Some(OpCode::IMul),
            5 => Some(OpCode::IDiv),
            6 => Some(OpCode::IRem),
            7 => Some(OpCode::Print),
            _ => None
        }
    }
}