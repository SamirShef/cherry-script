use crate::compiler::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Id(String, Location),
    
    // keywords
    Var(Location),
    Const(Location),
    Print(Location),
    
    // literals
    Int(i64, Location),
    Float(f64, Location),
    Str(String, Location),
    Char(char, Location),

    // operators
    Assign(Location),
    Gt(Location),
    GtEq(Location),
    Lt(Location),
    LtEq(Location),
    Eq(Location),
    NotEq(Location),
    Not(Location),
    And(Location),
    Or(Location),
    LogicalAnd(Location),
    LogicalOr(Location),
    Plus(Location),
    Minus(Location),
    Star(Location),
    Slash(Location),
    Percent(Location),
    Semi(Location),
    Colon(Location),
    Dot(Location),
    Comma(Location),
}