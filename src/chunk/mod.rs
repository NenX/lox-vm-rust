mod debug;
mod opcode;
pub use opcode::*;
pub use OpCode::*;

use crate::value::Value;
pub struct Chunk {
    code: Vec<OpCode>,
    lines: Vec<usize>,
    count: usize,
    constants: Vec<Value>,
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            lines: vec![],
            count: 0,
            constants: vec![],
        }
    }
    pub fn disassemble(&self, name: &str) {
        debug::disassemble_chunk(self, name);
    }
    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
        self.count += 1;
    }
    pub fn write_constant(&mut self, value: Value, line: usize) {
        let idx = self.add_constant(value);
        self.write_chunk(OPCONSTANT, line);
        self.write_chunk(OPVALUEIDX(idx), line);
    }
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
