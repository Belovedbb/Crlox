use std::{convert::TryInto};
use crate::compiler::Compiler;
use crate::scanner::Scanner;
use crate::value::{Value, ValueArray};
use crate::debug::{disassemble_chunk};
use crate::chunk::{Chunk, Opcode};

const STACK_MAX: usize = 254;

#[allow(dead_code, non_camel_case_types)]
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
}
pub struct VirtualMachine<'a> {
    chunk: Option<&'a Chunk>,
    stack: [Value;STACK_MAX],
    stack_top: usize,
    ip: usize
}

impl<'a> VirtualMachine<'a> {

    pub fn init_virtual_machine() -> Self {
        VirtualMachine {
            chunk: None,
            stack: [0.0;STACK_MAX],
            stack_top: 0,
            ip: 0
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    pub fn interpret(&mut self, content: &str, chunk: &'a mut Chunk) -> InterpretResult {
        //let mut chunk = Chunk::init_chunk();
        let mut sc = Scanner::init_scanner(content);
        let mut compiler = Compiler::init_compiler(&mut sc, chunk);
        if !compiler.compile() {
            return InterpretResult::INTERPRET_COMPILE_ERROR;
        }
        self.chunk = Some(chunk);
        self.run()
    }

    // pub fn interpret_(&mut self, content: &str) -> InterpretResult {
    //     InterpretResult::INTERPRET_OK
    // }
    
    fn inc_fn<F>(chunk: Option<&'a Chunk>, read_ip: &mut F) -> u8 where F: (FnMut() -> usize) {
        *chunk.unwrap().get_code().get(read_ip()).unwrap()
    }

    pub fn run(&mut self) -> InterpretResult {
        for i in 0..self.stack_top {
            print!("[");
            ValueArray::print_value(&self.stack[i]);
            print!("]")
        }
        println!("");
        disassemble_chunk(&self.chunk.unwrap(), "test chunk");
        let mut p = self.ip;
        let mut read_ip_increment = || { 
            let temp = p;
            p += 1;
            temp 
        };
        
        loop {
           let instruction = VirtualMachine::inc_fn(self.chunk, &mut read_ip_increment);
            {
                match instruction.try_into() {
                    Ok(Opcode::OP_RETURN) => {
                        ValueArray::print_value(&self.pop());
                        return InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_CONSTANT) => {
                        let constant: &Value = match self.chunk {
                            Some(ch) => ch.get_constants().get_values()
                            .get(VirtualMachine::inc_fn(self.chunk, &mut read_ip_increment) as usize),
                            None => Some(&0.0)
                        }.unwrap();
                        ValueArray::print_value(constant);
                        self.push(*constant);
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_NEGATE) => {
                        let op = -self.pop();
                        self.push(op);
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_ADD) => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a + b);
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_MULTIPLY) => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a * b);
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_SUBTRACT) => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a - b);
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_DIVIDE) => {
                        let b = self.pop();
                        let a = self.pop();
                        self.push(a / b);
                        InterpretResult::INTERPRET_OK
                    }
                    _ => return InterpretResult::INTERPRET_COMPILE_ERROR
                };
            }
        }
    }
}
