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
