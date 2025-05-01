use crate::{chunk::Chunk, Compiler, VM};

#[derive(Debug, Clone, Copy)]
pub enum InterpretErr {
    CompileError,
    RuntimeError,
}

pub fn interpret(source: &str) -> Result<(), InterpretErr> {
    let mut chunk = Chunk::new();
    let mut compiler = Compiler::new(&mut chunk, source);
    if !compiler.compile() {
        return Err(InterpretErr::CompileError);
    }
    chunk.disassemble("after compile");
    let mut vm = VM::new(&chunk);
    if let Err(e) = vm.run() {
        return Err(InterpretErr::RuntimeError);
    }
    Ok(())
}

#[test]
fn test() {
    let source = "nil";
    let r = interpret(source);
    println!("result: {:?}", r);
}
