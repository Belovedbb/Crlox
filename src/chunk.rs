use std::convert::TryFrom;

use crate::value::{Value, ValueArray};

// Bytecode repr

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    OP_RETURN = 0, // return from a function
    OP_CONSTANT = 1,
    OP_NEGATE = 2,
    OP_ADD = 3,
    OP_SUBTRACT = 4,
    OP_MULTIPLY = 5,
    OP_DIVIDE = 6,
}

impl TryFrom<u8> for Opcode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            val if val == Opcode::OP_RETURN as u8 => Ok(Opcode::OP_RETURN),
            val if val == Opcode::OP_CONSTANT as u8 => Ok(Opcode::OP_CONSTANT),
            val if val == Opcode::OP_NEGATE as u8 => Ok(Opcode::OP_NEGATE),
            val if val == Opcode::OP_ADD as u8 => Ok(Opcode::OP_ADD),
            val if val == Opcode::OP_SUBTRACT as u8 => Ok(Opcode::OP_SUBTRACT),
            val if val == Opcode::OP_MULTIPLY as u8 => Ok(Opcode::OP_MULTIPLY),
            val if val == Opcode::OP_DIVIDE as u8 => Ok(Opcode::OP_DIVIDE),
            _ => Err(())
        }
    }
}

// A representation of the bytecode format for cr virtual machine
pub struct Chunk {
    code: Vec<u8>,
    constants: ValueArray,
    lines: Vec<usize>
}

impl Chunk {

    pub fn init_chunk() -> Self {
        Chunk {
            code: Vec::new(),
            constants: ValueArray::init_value(),
            lines: Vec::new()
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value(value);
        self.constants.get_values().len() - 1
    }

    pub fn get_code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn get_line(&self) -> &Vec<usize> {
        &self.lines
    }

    pub fn get_constants(&self) -> &ValueArray {
        &self.constants
    }

}
