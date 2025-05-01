use crate::value::Value;

use super::{Chunk, OpCode};
use OpCode::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
    }
    println!("== {} ==", name);
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let byte = &chunk.code[offset];
    match byte {
        OPCONSTANT => constant_instruction("OPCONSTANT", chunk, offset),
        OPNIL => simple_instruction("OPNIL", offset),
        OPTRUE => simple_instruction("OPTRUE", offset),
        OPFALSE => simple_instruction("OPFALSE", offset),
        OPRETURN => simple_instruction("OPRETURN", offset),
        OPEQUAL => simple_instruction("OPEQUAL", offset),
        OPGREATER => simple_instruction("OPGREATER", offset),
        OPLESS => simple_instruction("OPLESS", offset),
        OPADD => simple_instruction("OPADD", offset),
        OPSUBTRACT => simple_instruction("OPSUBTRACT", offset),
        OPMULTIPLY => simple_instruction("OPMULTIPLY", offset),
        OPDIVIDE => simple_instruction("OPDIVIDE", offset),
        OPNOT => simple_instruction("OPNOT", offset),
        OPNEGATE => simple_instruction("OPNEGATE", offset),
        _ => todo!(),
    }
}
fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{:04}   {:16}", offset, name);
    offset + 1
}
fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let const_idx = chunk.code.get(offset + 1).expect("const_idx");
    let idx = const_idx.as_value_idx();
    let constant = chunk.constants[idx];
    println!("{:04}   {:16} {:?}", offset, name, constant);
    offset + 2
}
#[test]
fn test() {
    let mut chunk = Chunk::new();
    chunk.write_constant(Value::Number(1.0), 1);
    chunk.write_chunk(OPNIL, 1);
    chunk.write_chunk(OPTRUE, 1);
    chunk.write_chunk(OPFALSE, 1);
    chunk.write_chunk(OPRETURN, 1);
    disassemble_chunk(&chunk, "test");
}
