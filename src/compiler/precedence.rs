use Precedence::*;
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // == !=
    PrecComparison, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary,
}
impl Precedence {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PrecNone,
            1 => PrecAssignment,
            2 => PrecOr,
            3 => PrecAnd,
            4 => PrecEquality,
            5 => PrecComparison,
            6 => PrecTerm,
            7 => PrecFactor,
            8 => PrecUnary,
            9 => PrecCall,
            10 => PrecPrimary,
            _ => panic!("Invalid precedence value"),
        }
    }
    pub fn next(self) -> Self {
        let value = self.as_u8() + 1;
        Precedence::from_u8(value)
    }
}
