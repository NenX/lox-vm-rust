use crate::{
    chunk::{debug, Chunk, OpCode},
    interpreter::InterpretErr,
    value::Value,
};
use InterpretErr::*;
pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    ip: usize,
}
impl<'a> VM<'a> {
    fn reset_stack(&mut self) {
        self.stack.clear();
    }
    fn read_byte(&mut self) -> OpCode {
        let byte = self.chunk.code[self.ip];
        self.ip += 1;
        byte
    }
    fn read_const(&mut self) -> Value {
        let idx = self.read_byte().as_value_idx();
        self.chunk.constants[idx]
    }
    fn push_value(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop_value(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
    fn binary_op(&mut self, op: OpCode) -> Result<(), InterpretErr> {
        let b = self.pop_value().as_number().map_err(|_| RuntimeError)?;
        let a = self.pop_value().as_number().map_err(|_| RuntimeError)?;

        match op {
            OpCode::OPEQUAL => self.push_value(Value::Bool(a == b)),
            OpCode::OPGREATER => self.push_value(Value::Bool(a > b)),
            OpCode::OPLESS => self.push_value(Value::Bool(a < b)),
            OpCode::OPADD => self.push_value(Value::Number(a + b)),
            OpCode::OPSUBTRACT => self.push_value(Value::Number(a - b)),
            OpCode::OPMULTIPLY => self.push_value(Value::Number(a * b)),
            OpCode::OPDIVIDE => self.push_value(Value::Number(a / b)),
            _ => return Err(RuntimeError),
        };
        return Ok(());
    }
    fn is_false(&self, value: Value) -> bool {
        match value {
            Value::Bool(b) => !b,
            Value::Nil => true,
            _ => false,
        }
    }
}
impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            stack: vec![],
            ip: 0,
        }
    }
    pub fn run(&mut self) -> Result<(), InterpretErr> {
        loop {
            if cfg!(debug_assertions) {
                println!("       {:?}", self.stack);

                debug::disassemble_instruction(self.chunk, self.ip);
            }

            let byte = self.read_byte();
            match byte {
                OpCode::OPCONSTANT => {
                    let v = self.read_const();
                    self.push_value(v)
                }
                OpCode::OPNIL => self.push_value(Value::Nil),
                OpCode::OPTRUE => self.push_value(Value::Bool(true)),
                OpCode::OPFALSE => self.push_value(Value::Bool(false)),
                OpCode::OPRETURN => {
                    let value = self.pop_value();
                    println!("return {:?}", value);
                    break;
                }
                OpCode::OPNOT => {
                    let value = self.pop_value();
                    self.push_value(Value::Bool(self.is_false(value)));
                }
                OpCode::OPEQUAL
                | OpCode::OPGREATER
                | OpCode::OPLESS
                | OpCode::OPMULTIPLY
                | OpCode::OPDIVIDE
                | OpCode::OPADD
                | OpCode::OPSUBTRACT => {
                    self.binary_op(byte)?;
                }
                _ => return Err(RuntimeError),
            }
        }
        Ok(())
    }
}
