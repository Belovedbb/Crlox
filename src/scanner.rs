

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE, COMMA, DOT,
    SLASH, STAR, PLUS, MINUS, SEMICOLON,
    
    NOT, NOT_EQUAL, LESS, LESS_EQUAL, EQUAL, EQUAL_EQUAL, GREATER,
    GREATER_EQUAL,

    IDENTIFIER, STRING, NUMBER,

    TRUE, FALSE, AND, OR, NIL, WHILE, FOR, CLASS, 
    IF, ELSE, RETURN, VAR, SUPER, THIS, FUN, PRINT,

    EOF, ERROR
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    line: usize,
    content: &'a str,
    content_len: usize,
    content_start: usize,
    type_: TokenType
}

impl<'a> Token<'a> {
    pub fn  init_token(content: &'a str) -> Self {
        Token {
            line: 0,
            content: content,
            content_len: 0,
            content_start: 0,
            type_: TokenType::ERROR
        }
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
    
    #[allow(dead_code)]
    pub fn get_len(&self) -> usize {
        self.content_len
    }
    pub fn get_start(&self) -> usize {
        self.content_start
    }
    pub fn get_type(&self) -> &TokenType {
        &self.type_
    }

    pub fn get_sized_content(&self) -> &str {
        self.content.get(self.content_start..self.content_len).unwrap()
    }

    pub fn set_line(&mut self, line: usize)  {
        self.line = line
    }
    pub fn set_len(&mut self, len: usize) {
        self.content_len = len
    }
    pub fn set_start(&mut self, start: usize) {
        self.content_start = start
    }
    pub fn set_type(&mut self, kind: TokenType)  {
        self.type_ = kind
    }
}

#[derive(Clone, Copy)]
pub struct Scanner<'a> {
    line: usize,
    content: &'a str,
    current: usize,
    start: usize,
}

impl<'a> Scanner<'a> {

    pub fn init_scanner(content: &'a str) -> Self {
        Scanner { line: 1, content: content, current: 0, start: 0 }
    }

    fn make_token(&self, kind: TokenType) -> Token<'a> {
        let mut token = Token::init_token(self.content);
        token.set_type(kind);
        token.set_start(self.start);
        token.set_len(self.current - self.start);
        token.set_line(self.line);
        token
    }

    fn error_token(&self, message: &'a str) -> Token<'a> {
        let mut token = Token::init_token(message);
        token.set_start(0);
        token.set_len(message.len());
        token.set_line(self.line);
        token
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_white_spaces();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::EOF)
        }
        let ch = self.advance();
        match ch {
            '(' => self.make_token(TokenType::LEFT_PAREN),
            ')' => self.make_token(TokenType::RIGHT_PAREN),
            '{' => self.make_token(TokenType::LEFT_BRACE),
            '}' => self.make_token(TokenType::RIGHT_BRACE),
            ',' => self.make_token(TokenType::COMMA),
            '.' => self.make_token(TokenType::DOT),
            '-' => self.make_token(TokenType::MINUS),
            ';' => self.make_token(TokenType::SEMICOLON),
            '*' => self.make_token(TokenType::STAR),
            '+' => self.make_token(TokenType::PLUS),
            '/' => self.make_token(TokenType::SLASH),
            '!' => {
                if self.match_('=') {
                    self.make_token(TokenType::NOT_EQUAL)
                }else {
                    self.make_token(TokenType::NOT)
                }
            },
            '=' => {
                if self.match_('=') {
                    self.make_token(TokenType::EQUAL_EQUAL)
                }else {
                    self.make_token(TokenType::EQUAL)
                }
            },
            '<' => {
                if self.match_('=') {
                    self.make_token(TokenType::LESS_EQUAL)
                }else {
                    self.make_token(TokenType::LESS)
                }
            },
            '>' => {
                if self.match_('=') {
                    self.make_token(TokenType::GREATER_EQUAL)
                }else {
                    self.make_token(TokenType::GREATER)
                }
            },
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => self.error_token("Unexpected Character")
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.content.len()
    }

    fn advance(&mut self) -> char {
        let val = self.content.chars().nth(self.current).unwrap();
        self.current += 1;
        val
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }
        if !(self.content.chars().nth(self.current ).unwrap() == expected) {
            return false
        }
        self.current += 1;
        return true;
    }

    fn skip_white_spaces(&mut self) {
        loop {
            let ch = self.peek_();
            match ch {
                ' ' | '\r' | '\t'  => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                },
                '/' => {
                    if self.peek_next_() == '/' {
                        while !self.is_at_end() && !(self.peek_() == '\n') {
                            self.advance();
                        }
                    }
                },
                _ => break
            }
        }
    }

    fn peek_(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.content.chars().nth(self.current).unwrap();
    }

    fn peek_next_(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.content.chars().nth(self.current + 1).unwrap();
    }

    fn string(&mut self) -> Token<'a> {
        while !self.is_at_end() && self.peek_() != '"' {
            if self.peek_() == '\n' {
                self.current += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return self.error_token("Unterminated string")
        }
        self.advance();
        self.make_token(TokenType::STRING)
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek_().is_digit(10) {
            self.advance();
        }
        if self.peek_() == '.' && self.peek_next_().is_digit(10) {
            self.advance();
            while self.peek_().is_digit(10) {
                self.advance();
            }
        }
        self.make_token(TokenType::NUMBER)
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.peek_().is_alphanumeric() || self.peek_() == '_' {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self ) -> TokenType {
        match self.content.chars().nth(self.start).unwrap() {
            'a' => self.check_keyword(1, 2, "and", TokenType::AND),
            'c' => self.check_keyword(1, 4, "class", TokenType::CLASS),
            'e' => self.check_keyword(1, 3, "else", TokenType::ELSE),
            'i' => self.check_keyword(1, 1, "if", TokenType::IF),
            'n' => self.check_keyword(1, 2, "il", TokenType::NIL),
            'o' => self.check_keyword(1, 1, "or", TokenType::OR),
            'p' => self.check_keyword(1, 4, "print", TokenType::PRINT),
            'r' => self.check_keyword(1, 5, "return", TokenType::RETURN),
            's' => self.check_keyword(1, 4, "super", TokenType::SUPER),
            'v' => self.check_keyword(1, 2, "are", TokenType::VAR),
            'w' => self.check_keyword(1, 4, "while", TokenType::WHILE),
            'f' => {
                if self.current - self.start > 1 {
                    match self.content.chars().nth(self.start + 1).unwrap() {
                        'a' =>  self.check_keyword(2, 3, "false", TokenType::FALSE),
                        'o' =>  self.check_keyword(2, 1, "for", TokenType::FOR),
                        'u' =>  self.check_keyword(2, 1, "fun", TokenType::FUN),
                        _ => TokenType::IDENTIFIER
                    }
                }else {
                    TokenType::IDENTIFIER
                }
            },
            't' => {
                if self.current - self.start > 1 {
                    match self.content.chars().nth(self.start + 1).unwrap() {
                        'h' =>  self.check_keyword(2, 2, "this", TokenType::THIS),
                        'r' =>  self.check_keyword(2, 2, "true", TokenType::TRUE),
                        _ => TokenType::IDENTIFIER
                    }
                }else {
                    TokenType::IDENTIFIER
                }
            }
            _ => TokenType::IDENTIFIER
        }
    }

    fn check_keyword(&self, start: usize, len: usize, value: &str, kind: TokenType) -> TokenType {
        if (self.current - self.start) == (start + len)  
        && self.content.get(self.start..self.current).unwrap() == value {
            return kind;
        }
        TokenType::IDENTIFIER
    }

}
