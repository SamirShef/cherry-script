pub enum OpCode {
    Push        = 0,
    Pop         = 1,
    Add         = 2,
    Sub         = 3,
    Mul         = 4,
    Div         = 5,
    Rem         = 6,
    StoreGlob   = 7,
    LoadGlob    = 8,
    Jmp         = 9,
    JmpIf       = 10,
    Call        = 11,
    Ret         = 12,
    StoreLoc    = 13,
    LoadLoc     = 14,
    Print       = 15,
}

impl OpCode {
    pub fn from_u8(val: u8) -> Option<OpCode> {
        match val {
            0   => Some(OpCode::Push),
            1   => Some(OpCode::Pop),
            2   => Some(OpCode::Add),
            3   => Some(OpCode::Sub),
            4   => Some(OpCode::Mul),
            5   => Some(OpCode::Div),
            6   => Some(OpCode::Rem),
            7   => Some(OpCode::StoreGlob),
            8   => Some(OpCode::LoadGlob),
            9   => Some(OpCode::Jmp),
            10  => Some(OpCode::JmpIf),
            11  => Some(OpCode::Call),
            12  => Some(OpCode::Ret),
            13  => Some(OpCode::StoreLoc),
            14  => Some(OpCode::LoadLoc),
            15  => Some(OpCode::Print),
            _   => None
        }
    }
}