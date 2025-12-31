#[derive(Debug, Clone)]
pub enum StackSlot {
    Int(i64),
    Float(f64)
}

impl StackSlot {
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            StackSlot::Int(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            StackSlot::Float(val) => Some(*val),
            _ => None
        }
    }
}