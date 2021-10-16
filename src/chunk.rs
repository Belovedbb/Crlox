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
    OP_NIL = 7,
    OP_TRUE = 8,
    OP_FALSE = 9,
    OP_NOT = 10,
    OP_EQUAL = 11,
    OP_GREATER = 12,
    OP_LESS = 13,
    OP_PRINT = 14,
    OP_POP = 15,
    OP_DEFINE_GLOBAL = 16,
    OP_GET_GLOBAL = 17,
    OP_SET_GLOBAL = 18,

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
            val if val == Opcode::OP_NIL as u8 => Ok(Opcode::OP_NIL),
            val if val == Opcode::OP_TRUE as u8 => Ok(Opcode::OP_TRUE),
            val if val == Opcode::OP_FALSE as u8 => Ok(Opcode::OP_FALSE),
            val if val == Opcode::OP_NOT as u8 => Ok(Opcode::OP_NOT),
            val if val == Opcode::OP_EQUAL as u8 => Ok(Opcode::OP_EQUAL),
            val if val == Opcode::OP_GREATER as u8 => Ok(Opcode::OP_GREATER),
            val if val == Opcode::OP_LESS as u8 => Ok(Opcode::OP_LESS),
            val if val == Opcode::OP_PRINT as u8 => Ok(Opcode::OP_PRINT),
            val if val == Opcode::OP_POP as u8 => Ok(Opcode::OP_POP),
            val if val == Opcode::OP_DEFINE_GLOBAL as u8 => Ok(Opcode::OP_DEFINE_GLOBAL),
            val if val == Opcode::OP_GET_GLOBAL as u8 => Ok(Opcode::OP_GET_GLOBAL),
            val if val == Opcode::OP_SET_GLOBAL as u8 => Ok(Opcode::OP_SET_GLOBAL),
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
