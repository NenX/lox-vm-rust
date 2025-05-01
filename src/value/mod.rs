#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
}
impl Value {
    pub fn as_number(&self) -> Result<f64,()> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(()),
        }
    }
    pub fn as_bool(&self) -> Result<bool,()> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(()),
        }
    }
}
