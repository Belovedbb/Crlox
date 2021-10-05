mod chunk;
mod common;
mod debug;
mod value;
mod compiler;
mod vm;
mod scanner;
mod parser;

use chunk::Chunk;
use chunk::Opcode;
use vm::InterpretResult;
use vm::VirtualMachine;
use std::{env, io, fs};

fn main() -> () {
    let x: Vec<_> = env::args().collect();
    if x.len() == 2 {
        run_file(&x[1])
    }else if x.len() < 2 {
        repl();
    }else {
        panic!("Error {}", -5);
    }
    
    //let mut vm = VirtualMachine::init_virtual_machine();
    // let mut chunk = Chunk::init_chunk();
    // let const_index = chunk.add_constant(4.3);//write to value array
    // chunk.write_chunk(Opcode::OP_CONSTANT as u8, 1); // write bytecode as constant
    // chunk.write_chunk(const_index as u8, 1);// prev bytecode operand
    // chunk.write_chunk(Opcode::OP_NEGATE as u8, 1);
    // //addition
    // let const_index = chunk.add_constant(4.3);//write to value array
    // chunk.write_chunk(Opcode::OP_CONSTANT as u8, 1); // write bytecode as constant
    // chunk.write_chunk(const_index as u8, 1);// prev bytecode operand
    // let const_index = chunk.add_constant(4.3);//write to value array
    // chunk.write_chunk(Opcode::OP_CONSTANT as u8, 1); // write bytecode as constant
    // chunk.write_chunk(const_index as u8, 1);// prev bytecode operand
    // chunk.write_chunk(Opcode::OP_ADD as u8, 1);

    // chunk.write_chunk(Opcode::OP_RETURN as u8, 2);
    // //disassemble_chunk(&chunk, "test chunk");
    // vm.interpret(&chunk);
}

fn repl() {
    let mut content = String::new();
    loop {
        println!("\nWelcome to CR REPL. Type in the source >>");
        io::stdin().read_line(&mut content).unwrap();
        run(content.trim());
    }
}

fn run_file(path: &str) {
    let content = fs::read_to_string(path).expect("path does not exist");
    let interpret_result = run(content.trim());
    match interpret_result {
        InterpretResult::INTERPRET_COMPILE_ERROR => panic!("Error 65 has occured"),
        InterpretResult::INTERPRET_RUNTIME_ERROR => panic!("Error 66 has occured"),
        InterpretResult::INTERPRET_OK => ()
    }
}

fn run(source: &str) -> InterpretResult {
    let mut vm = VirtualMachine::init_virtual_machine();
    let mut chunk = Chunk::init_chunk();
    vm.interpret(source, &mut chunk);
    //disassemble_chunk(&chunk, "test chunk");
    InterpretResult::INTERPRET_OK
}
