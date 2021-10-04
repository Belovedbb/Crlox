
use crate::scanner::{Token, TokenType};
use crate::chunk::{Chunk, Opcode};
use crate::value::Value;
use super::scanner::Scanner;
use crate::parser::{Precedence, get_rule};



pub struct Compiler<'a> {
    current: Token<'a>,
    prev: Token<'a>,
    scanner: &'a mut Scanner<'a>,
    chunk:  &'a mut Chunk,
    had_error: bool,
    panic_mode: bool
}

impl <'a> Compiler<'a>  {

    pub fn get_prev(&self) -> &Token {
        &self.prev
    }

    pub fn init_compiler(scanner: &'a mut Scanner<'a>, chunk: &'a mut Chunk) -> Self {
        Compiler { 
            current: Token::init_token(""),
            prev: Token::init_token(""),
            chunk: chunk,
            scanner: scanner,
            had_error: false,
            panic_mode: false
     }
    }

    pub fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(&TokenType::EOF, "Expect end of expression.");
        self.end_compiler();
        return !self.had_error;
    }

    pub fn expression(&mut self) {
        self.parse_precedence(&Precedence::PREC_ASSIGNMENT);
    }

    pub fn consume(&mut self, token_type: &TokenType, message: &str) {
        if *self.current.get_type() == *token_type {
            self.advance();
        }
        self.error_at_current(&String::from(self.current.get_sized_content()));
    }

    pub fn parse_precedence(&mut self, precedence: &Precedence) {
        self.advance();
        let rule = get_rule(self.prev.get_type());
        let prefix_rule = rule.get_prefix();
         match *prefix_rule {
            Some(x) => x(self),
            None => return self.error("Expect expression")
        };
        while *precedence as usize <= *get_rule(self.current.get_type()).get_precedence() as usize {
            self.advance();
            let inf_rule  = get_rule(self.prev.get_type()).get_infx().unwrap();
            inf_rule(self);
          }
    }

    pub fn emit_constant(&mut self, value: Value) {
        let index = self.chunk.add_constant(value);
        if index as u8 > u8::MAX {
            self.error("Too many constant in chunk")
        }
        self.emit_bytes(Opcode::OP_CONSTANT as u8, index as u8)
    }

    pub fn make_constant(&mut self, value: Value) -> u8 {
        let index = self.chunk.add_constant(value);
        index as u8
    }

    pub fn end_compiler(&mut self) {
        self.emit_return()
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(Opcode::OP_RETURN as u8)
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.chunk.write_chunk(byte, self.prev.get_line());
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }
    
    fn advance(&mut self) {
        self.prev = self.current;
        //let mut line = 0;
        loop {
            self.current = self.scanner.scan_token();
            if *self.current.get_type() != TokenType::ERROR {
                break;
            }
            /*boxing in String to prev same space mut and immutable *self* */
            self.error_at_current(&String::from(self.current.get_sized_content()));
        }
        
    }

    //parser error handling 
    fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current, message);
        self.had_error = true;
        self.panic_mode = true;
    }

    fn error(&mut self, message: &str) {
        self.error_at(&self.prev, message);
        self.had_error = true;
        self.panic_mode = true;
    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.panic_mode {
            ()
        }
        print!("\n[line {} ] Error ", token.get_line());
        if *token.get_type() == TokenType::EOF {
            print!(" at end")
        } else if *token.get_type() == TokenType::ERROR {
    
        } else {
            print!(" at {} ", token.get_start())
        }
        println!("{}", message);
    }
}

