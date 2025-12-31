pub enum Token {
    Id(String),
    
    // keywords
    Var,
    
    // literals
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),

    // operators
    Assign,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Eq,
    NotEq,
    Not,
    And,
    Or,
    LogicalAnd,
    LogicalOr,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}