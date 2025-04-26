#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    OPCONSTANT,
    OPNIL,
    OPTRUE,
    OPFALSE,
    OPRETURN,
    OPEQUAL,
    OPGREATER,
    OPLESS,
    OPADD,
    OPSUBTRACT,
    OPMULTIPLY,
    OPDIVIDE,
    OPNOT,
    OPNEGATE,
    OPVALUEIDX(usize),
}
impl OpCode {
    pub fn to_byte(&self) -> u8 {
        let disc = std::mem::discriminant(self);
        let value = unsafe { *(&disc as *const _ as *const u8) }; // 可能存在风险
        return value as u8;
    }
    pub fn as_value_idx(&self) -> usize {
        match self {
            OpCode::OPVALUEIDX(idx) => idx.clone(),
            _ => panic!("not a value idx"),
        }
    }
}
