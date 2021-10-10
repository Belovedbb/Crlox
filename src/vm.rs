use std::{convert::TryInto};
use crate::compiler::Compiler;
use crate::scanner::Scanner;
pub(crate) use crate::value::{Value, ValueArray, ValueType, AsValue, is_obj_type};
use crate::debug::{disassemble_chunk};
use crate::chunk::{Chunk, Opcode};
use crate::object::{ObjString, Obj, ObjType};

const STACK_MAX: usize = 256;

#[allow(dead_code, non_camel_case_types)]
pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
}
pub struct VirtualMachine<'a> {
    chunk: Option<&'a Chunk>,
    stack: Vec<Value>,
    stack_top: usize,
    ip: usize
}

impl<'a> VirtualMachine<'a> {

    pub fn init_virtual_machine() -> Self {
        VirtualMachine {
            chunk: None,
            stack: Vec::with_capacity(STACK_MAX),
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
        let self_stack = self.stack.clone();
        self_stack[self.stack_top].clone()
    }

    pub fn peek(&mut self, dist: usize) -> &Value {
        &self.stack[self.stack_top - (1 + dist)]
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
        println!();
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
                            None => Some(&number_val!(0.0))
                        }.unwrap();
                        ValueArray::print_value(constant);
                        self.push((*constant).clone());
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_NEGATE) => {
                        if !is_number!(*self.peek(0)) {
                            self.runtime_error("Operand must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }
                        let op = -as_number!(self.pop());
                        self.push(number_val!(op));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_ADD) => {
                        if is_str!(*self.peek(0)) && is_str!(*self.peek(1)) {
                            self.concatenate();
                            InterpretResult::INTERPRET_OK
                        }
                        else if is_number!(*self.peek(0)) &&  is_number!(*self.peek(1)) {
                            let b = self.pop();
                            let a = self.pop();
                            self.push(number_val!(as_number!(a) + as_number!(b)));
                            InterpretResult::INTERPRET_OK
                        }else {
                            self.runtime_error("Operands must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }
                    },
                    Ok(Opcode::OP_MULTIPLY) => {
                        if !is_number!(*self.peek(0)) ||  !is_number!(*self.peek(1)) {
                            self.runtime_error("Operands must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }

                        let b = self.pop();
                        let a = self.pop();
                        self.push(number_val!(as_number!(a) * as_number!(b)));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_SUBTRACT) => {
                        if !is_number!(*self.peek(0)) ||  !is_number!(*self.peek(1)) {
                            self.runtime_error("Operands must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }

                        let b = self.pop();
                        let a = self.pop();
                        self.push(number_val!(as_number!(a) - as_number!(b)));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_DIVIDE) => {
                        if !is_number!(*self.peek(0)) ||  !is_number!(*self.peek(1)) {
                            self.runtime_error("Operands must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }
                        let b = self.pop();
                        let a = self.pop();
                        self.push(number_val!(as_number!(a) / as_number!(b)));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_TRUE) => {
                        self.push(boolean_val!(true));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_FALSE) => {
                        self.push(boolean_val!(false));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_NIL) => {
                        self.push(nill!());
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_NOT) => {
                        let value:Value = self.pop();
                        self.push(boolean_val!(self.is_falsey(value)));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_EQUAL) => {
                        let b:Value = self.pop();
                        let a:Value = self.pop();
                        self.push(boolean_val!(self.values_equal(a, b)));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_GREATER) => {
                        if !is_number!(*self.peek(0)) ||  !is_number!(*self.peek(1)) {
                            self.runtime_error("Operands must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }
                        let b = self.pop();
                        let a = self.pop();
                        self.push(boolean_val!(as_number!(a) > as_number!(b)));
                        InterpretResult::INTERPRET_OK
                    },
                    Ok(Opcode::OP_LESS) => {
                        if !is_number!(*self.peek(0)) ||  !is_number!(*self.peek(1)) {
                            self.runtime_error("Operands must be a number");
                            return InterpretResult::INTERPRET_RUNTIME_ERROR;
                        }
                        let b = self.pop();
                        let a = self.pop();
                        self.push(boolean_val!(as_number!(a) < as_number!(b)));
                        InterpretResult::INTERPRET_OK
                    },
                    _ => return InterpretResult::INTERPRET_COMPILE_ERROR
                };
            }
        }
    }

    fn is_falsey(&self, value: Value) -> bool {
        is_nill!(value) || (is_boolean!(value) && !as_boolean!(value))
    }

    fn concatenate(&mut self) {
        let b_ = String::from(as_str_raw!(self.pop()));
        let a_ = String::from(as_str_raw!(self.pop()));
        let res_= format!("{}{}", a_, b_);
        let obj_res = ObjString::from(res_);
        self.push(obj_val!(Box::from(obj_res)))
    }

    fn values_equal(&self, a: Value, b: Value) -> bool {
        if *a.get_type_ref() != *a.get_type_ref(){
            return false;
        }
        match *a.get_type_ref() {
            ValueType::VAL_BOOLEAN => as_boolean!(a) == as_boolean!(b),
            ValueType::VAL_NIL => true,
            ValueType::VAL_NUMBER => as_number!(a) == as_number!(b),
            ValueType::VAL_OBJ => {
                let a = String::from(as_str_raw!(a));
                let b = String::from(as_str_raw!(b));
                a == b
            }
            
        }
    }

    fn runtime_error(&self, message: &str ) {
        println!("{}", message);
    }
}
