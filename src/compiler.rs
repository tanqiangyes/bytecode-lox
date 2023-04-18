use crate::chunk::Chunk;
use crate::object::Object;
use crate::opcode::OpCode;
use crate::precedence::Precedence;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use crate::vm::InterpretResult;
use std::cell::RefCell;

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner,
    chunk: &'a mut Chunk,
    // rules: Vec<ParseRule<'a>>,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        // let rules =
        Self {
            parser: Default::default(),
            scanner: Scanner::new(""),
            chunk,
            // rules,
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<(), InterpretResult> {
        self.scanner = Scanner::new(source);
        self.advance();

        while !self.is_match(TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();
        if *self.parser.had_error.borrow() {
            Err(InterpretResult::CompileError)
        } else {
            Ok(())
        }
    }

    fn declaration(&mut self) {
        self.statement();
    }

    fn statement(&mut self) {
        if self.is_match(TokenType::Print) {
            self.print_statement();
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::SemiColon, "Expect ';' after value.");
        self.emit_code(OpCode::Print);
    }

    fn is_match(&mut self, ttype: TokenType) -> bool {
        if !self.check(ttype) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&mut self, ttype: TokenType) -> bool {
        self.parser.current.is(ttype)
    }

    fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();

        loop {
            self.parser.current = self.scanner.scan_token();
            if !self.parser.current.is(TokenType::Error) {
                break;
            }

            self.error_at_current(&self.parser.current.as_string())
        }
    }

    fn consume(&mut self, ttype: TokenType, message: &str) {
        if self.parser.current.is(ttype) {
            self.advance();
            return;
        }
        self.error_at_current(message)
    }

    fn emit_byte(&mut self, bytes: u8) {
        self.chunk.write(bytes, self.parser.previous.line);
    }

    fn emit_code(&mut self, code: OpCode) {
        self.chunk.write_opcode(code, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, code: OpCode, operand: u8) {
        self.emit_code(code);
        self.emit_byte(operand);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return.into());
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        if let Some(constant) = self.chunk.add_constant(value) {
            constant
        } else {
            self.error_at_previous("Too many constants in one chunk.");
            0
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant, constant);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        #[cfg(feature = "debug_print_code")]
        if !*self.parser.had_error.borrow() {
            self.chunk.disassemble("disassemble code")
        }
    }

    fn binary(&mut self) {
        let operator_type = self.parser.previous.ttype;
        let rule = self.get_rule(operator_type);

        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::BangEqual => self.emit_code(OpCode::BangEqual),
            TokenType::Equal => self.emit_byte(OpCode::Equal.into()),
            TokenType::Greater => self.emit_byte(OpCode::Greater.into()),
            TokenType::GreaterEqual => self.emit_code(OpCode::GreaterEqual),
            TokenType::Less => self.emit_byte(OpCode::Less.into()),
            TokenType::LessEqual => self.emit_code(OpCode::LessEqual),
            TokenType::Plus => self.emit_byte(OpCode::Add.into()),
            TokenType::Minus => self.emit_byte(OpCode::Subtract.into()),
            TokenType::Star => self.emit_byte(OpCode::Multiply.into()),
            TokenType::Slash => self.emit_byte(OpCode::Divide.into()),
            _ => {}
        }
    }

    fn literal(&mut self) {
        match self.parser.previous.ttype {
            TokenType::Nil => self.emit_byte(OpCode::Nil.into()),
            TokenType::True => self.emit_byte(OpCode::True.into()),
            TokenType::False => self.emit_byte(OpCode::False.into()),
            _ => {}
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn string(&mut self) {
        let len = self.parser.previous.lexeme.len() - 1;
        let value = self.parser.previous.as_string()[1..len].to_string();
        self.emit_constant(Value::Obj(Object::Str(value)))
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.ttype;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => {
                self.emit_byte(OpCode::Negate.into());
            }
            TokenType::Bang => {
                self.emit_byte(OpCode::Not.into());
            }
            _ => self.error_at_current("Unsupported Operand type."),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(prefix_rule) = self.get_rule(self.parser.previous.ttype).prefix {
            prefix_rule(self);

            while precedence <= self.get_rule(self.parser.current.ttype).precedence {
                self.advance();
                if let Some(infix_rule) = self.get_rule(self.parser.previous.ttype).infix {
                    infix_rule(self)
                }
            }
        } else {
            self.error_at_previous("Expected expression.");
        }
    }

    fn get_rule(&self, ttype: TokenType) -> ParseRule {
        match ttype {
            TokenType::LeftParen => ParseRule {
                prefix: Some(|c| c.grouping()),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Minus => ParseRule {
                prefix: Some(|c| c.unary()),
                infix: Some(|c| c.binary()),
                precedence: Precedence::Term,
            },
            TokenType::Plus => ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Term,
            },
            TokenType::Slash | TokenType::Star => ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Factor,
            },
            TokenType::Number => ParseRule {
                prefix: Some(|c| c.number()),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::False | TokenType::True | TokenType::Nil => ParseRule {
                prefix: Some(|c| c.literal()),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::Bang => ParseRule {
                prefix: Some(|c| c.unary()),
                infix: None,
                precedence: Precedence::None,
            },
            TokenType::BangEqual | TokenType::Equal => ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Equality,
            },
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => ParseRule {
                prefix: None,
                infix: Some(|c| c.binary()),
                precedence: Precedence::Comparison,
            },
            TokenType::String => ParseRule {
                prefix: Some(|c| c.string()),
                infix: None,
                precedence: Precedence::None,
            },
            _ => ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.current.clone(), message);
    }

    fn error_at_previous(&mut self, message: &str) {
        self.error_at(self.parser.previous.clone(), message);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if *self.parser.panic_mode.borrow() {
            return;
        }
        self.parser.panic_mode.replace(true);
        eprint!("[line {}] Error", token.line);

        if token.is(TokenType::Eof) {
            eprint!(" at end.");
        } else if token.is(TokenType::Error) {
        } else {
            eprint!(" at '{}'", token.as_string());
        }

        eprintln!(": {message}");

        self.parser.had_error.replace(true);
    }
}

#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

type ParseFn = fn(&mut Compiler);

struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}
