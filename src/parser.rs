use crate::{chunk::Opcode, compiler::Compiler, scanner::TokenType};


type ParseType = fn(&mut Compiler<'_>) -> ();

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Precedence {
    PREC_NONE = 0,
    PREC_ASSIGNMENT = 1,
    PREC_OR = 2,
    PREC_AND = 3,
    PREC_EQUAL = 4,
    PREC_COMPARISON = 5,
    PREC_TERM = 6,
    PREC_FACTOR = 7,
    PREC_UNARY = 8,
    PREC_CALL = 9,
    PREC_PRIMARY = 10
}

pub struct ParseRule {
    precedence: Precedence,
    prefix: Option<ParseType>,
    infix: Option<ParseType>
}

impl ParseRule {
    pub fn get_prefix(&self) -> &Option<ParseType> {
        &self.prefix
    }

    pub fn get_infx(&self) -> &Option<ParseType> {
        &self.infix
    }

    pub fn get_precedence(&self) -> &Precedence {
        &self.precedence
    }

}

impl Default for ParseRule {
    fn default() -> Self {
        ParseRule { precedence: Precedence::PREC_NONE, prefix: None, infix: None }
    }
}

pub fn get_rule(token_type: &TokenType) -> ParseRule {
    match *token_type {
        TokenType::NUMBER => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: Some(parse_number),
                infix: None
            }
        },
        TokenType::LEFT_PAREN => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: Some(parse_grouping),
                infix: None
            }
        },
        TokenType::RIGHT_PAREN => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::LEFT_BRACE => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: Some(parse_grouping),
                infix: None
            }
        },
        TokenType::RIGHT_BRACE => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::COMMA => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::DOT => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::MINUS => {
            ParseRule {
                precedence: Precedence::PREC_TERM,
                prefix: Some(parse_unary),
                infix: Some(parse_binary)
            }
        },
        TokenType::PLUS => {
            ParseRule {
                precedence: Precedence::PREC_TERM,
                prefix: None,
                infix: Some(parse_binary)
            }
        },
        TokenType::SEMICOLON => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::SLASH => {
            ParseRule {
                precedence: Precedence::PREC_FACTOR,
                prefix: None,
                infix: Some(parse_binary)
            }
        },
        TokenType::NOT => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::NOT_EQUAL => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::EQUAL => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::EQUAL_EQUAL => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::GREATER => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::GREATER_EQUAL => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::LESS => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::LESS_EQUAL => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::IDENTIFIER => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::STRING => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::AND => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::CLASS => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::ELSE => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::FALSE => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::FOR => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::FUN => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::IF => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::NIL => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::OR => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::PRINT => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::RETURN => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::SUPER => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::THIS => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::TRUE => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::VAR => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::WHILE => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::ERROR => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },
        TokenType::EOF => {
            ParseRule {
                precedence: Precedence::PREC_NONE,
                prefix: None,
                infix: None
            }
        },

        _ => ParseRule::default()
        
    }
}

fn parse_binary(compiler: &mut Compiler) -> () {
    let op = (*compiler.get_prev().get_type()).clone();
    let rule = get_rule(&op);
    compiler.parse_precedence(&rule.precedence);
    match op {
        TokenType::PLUS => compiler.emit_byte(TokenType::PLUS as u8),
        TokenType::MINUS => compiler.emit_byte(TokenType::MINUS as u8),
        TokenType::STAR => compiler.emit_byte(TokenType::STAR as u8),
        TokenType::SLASH => compiler.emit_byte(TokenType::SLASH as u8),
        _ => ()
    }
}

fn parse_grouping(compiler: &mut Compiler) -> () {
    compiler.expression();
    compiler.consume(&TokenType::RIGHT_PAREN, "Expect ')' after expression");
    
}

fn parse_number(compiler: &mut Compiler) -> ()  {
    compiler.emit_constant(compiler.get_prev().get_sized_content().parse::<f64>().unwrap());
}

fn parse_unary(compiler: &mut Compiler) -> () {
    compiler.expression();
    match *compiler.get_prev().get_type() {
        TokenType::MINUS => compiler.emit_byte(Opcode::OP_NEGATE as u8),
        _ => ()
    }
}