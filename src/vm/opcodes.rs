pub enum OpCode {
    Push        = 0,
    Pop         = 1,
    IAdd        = 2,
    ISub        = 3,
    IMul        = 4,
    IDiv        = 5,
    IRem        = 6,
    FAdd        = 7,
    FSub        = 8,
    FMul        = 9,
    FDiv        = 10,
    FRem        = 11,
    StoreGlob   = 12,
    LoadGlob    = 13,
    Jmp         = 14,
    JmpIf       = 15,
    Call        = 16,
    Ret         = 17,
    StoreLoc    = 18,
    LoadLoc     = 19,
    Print       = 20,
}

impl OpCode {
    pub fn from_u8(val: u8) -> Option<OpCode> {
        match val {
            0   => Some(OpCode::Push),
            1   => Some(OpCode::Pop),
            2   => Some(OpCode::IAdd),
            3   => Some(OpCode::ISub),
            4   => Some(OpCode::IMul),
            5   => Some(OpCode::IDiv),
            6   => Some(OpCode::IRem),
            7   => Some(OpCode::FAdd),
            8   => Some(OpCode::FSub),
            9   => Some(OpCode::FMul),
            10  => Some(OpCode::FDiv),
            11  => Some(OpCode::FRem),
            12  => Some(OpCode::StoreGlob),
            13  => Some(OpCode::LoadGlob),
            14  => Some(OpCode::Jmp),
            15  => Some(OpCode::JmpIf),
            16  => Some(OpCode::Call),
            17  => Some(OpCode::Ret),
            18  => Some(OpCode::StoreLoc),
            19  => Some(OpCode::LoadLoc),
            20  => Some(OpCode::Print),
            _   => None
        }
    }
}