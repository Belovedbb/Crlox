use std::convert::TryInto;

use crate::chunk::Chunk;
use crate::chunk::Opcode;
use crate::value::ValueArray;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < chunk.get_code().len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    println!("offset {off:=>0}", off = offset);
    if offset > 0 &&  chunk.get_line()[offset] == chunk.get_line()[offset - 1] {
        println!("|")
    }else {
        println!("line -> {}", chunk.get_line()[offset]);
    }
    let val = chunk.get_code().get(offset).unwrap();
    match (*val).try_into() {
        Ok(Opcode::OP_RETURN) => simple_instruction("OP_RETURN", offset),
        Ok(Opcode::OP_CONSTANT) => constant_instruction("OP_CONSTANT", chunk, offset),
        Ok(Opcode::OP_NEGATE) => simple_instruction("OP_NEGATE", offset),
        Ok(Opcode::OP_ADD) => simple_instruction("OP_ADD", offset),
        Ok(Opcode::OP_MULTIPLY) => simple_instruction("OP_MULTIPLY", offset),
        Ok(Opcode::OP_SUBTRACT) => simple_instruction("OP_SUBTRACT", offset),
        Ok(Opcode::OP_DIVIDE) => simple_instruction("OP_DIVIDE", offset),
        Ok(Opcode::OP_NIL) => simple_instruction("OP_NIL", offset),
        Ok(Opcode::OP_TRUE) => simple_instruction("OP_TRUE", offset),
        Ok(Opcode::OP_FALSE) => simple_instruction("OP_FALSE", offset),
        Ok(Opcode::OP_NOT) => simple_instruction("OP_NOT", offset),
        Ok(Opcode::OP_EQUAL) => simple_instruction("OP_EQUAL", offset),
        Ok(Opcode::OP_GREATER) => simple_instruction("OP_GREATER", offset),
        Ok(Opcode::OP_LESS) => simple_instruction("OP_LESS", offset),
        Ok(Opcode::OP_PRINT) => simple_instruction("OP_PRINT", offset),
        Ok(Opcode::OP_POP) => simple_instruction("OP_POP", offset),
        Ok(Opcode::OP_DEFINE_GLOBAL) => constant_instruction("OP_DEFINE_GLOBAL", chunk, offset),
        Ok(Opcode::OP_GET_GLOBAL) => constant_instruction("OP_GET_GLOBAL", chunk, offset),
        Ok(Opcode::OP_SET_GLOBAL) => constant_instruction("OP_SET_GLOBAL", chunk, offset),
        _ => offset + 1
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.get_code().get(offset + 1).unwrap();
    println!("{} => index {}", name, *constant_index);
    let constant = chunk.get_constants().get_values().get(*constant_index as usize).unwrap();
    ValueArray::print_value(constant);
    offset + 2
}
