use std::mem;

use crate::value::Value;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpNop,
    OpUnit,
    OpTrue,
    OpFalse,
    OpConst,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpRem,
    OpPow,
    OpNeg,
    OpLt,
    OpGt,
    OpLtEq,
    OpGtEq,
    OpEqual,
    OpNotEq,
    OpLoop,
    OpJump,
    OpBranch,
    OpGet,
    OpPop,
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let max = Self::OpPop as u8;
        if value > max {
            return Err(());
        } else {
            return Ok(unsafe { mem::transmute(value) });
        }
    }
}

pub struct Chunk {
    code: Vec<u8>,
    vals: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            vals: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn code(&self, idx: usize) -> u8 {
        self.code[idx]
    }

    pub fn value(&self, idx: usize) -> Value {
        self.vals[idx].clone()
    }

    pub fn write(&mut self, opcode: OpCode) {
        self.code.push(opcode as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add(&mut self, value: Value) -> usize {
        let idx = self.vals.len();
        if value.is_str() {
            let target = value.clone().as_str();
            for (i, val) in self.vals.iter().enumerate() {
                if val.is_str() && val.clone().as_str() == target {
                    return i;
                }
            }
        }
        self.vals.push(value);
        idx
    }
}
