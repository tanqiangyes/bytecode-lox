use crate::chunk::Chunk;
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
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");
        // let mut line = 0;
        // loop {
        //     let token = scanner.scan_token();
        //     if token.line != line {
        //         print!("{:4}  ", token.line);
        //         line = token.line;
        //     } else {
        //         print!("    |  ");
        //     }
        //     println!("{:10?}  '{}'", token.ttype, token.lexeme);
        //
        //     if token.is(TokenType::Eof) {
        //         break;
        //     }
        // }
        self.end_compiler();
        if *self.parser.had_error.borrow() {
            Err(InterpretResult::CompileError)
        } else {
            Ok(())
        }
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

    fn emit_bytes(&mut self, code: OpCode, operand: u8) {
        self.emit_byte(code.into());
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
            TokenType::Plus => self.emit_byte(OpCode::Add.into()),
            TokenType::Minus => self.emit_byte(OpCode::Subtract.into()),
            TokenType::Star => self.emit_byte(OpCode::Multiply.into()),
            TokenType::Slash => self.emit_byte(OpCode::Divide.into()),
            _ => {}
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn number(&mut self) {
        let value = self.parser.previous.lexeme.parse::<Value>().unwrap();
        self.emit_constant(value);
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.ttype;

        self.parse_precedence(Precedence::Unary);

        if operator_type == TokenType::Minus {
            self.emit_byte(OpCode::Negate.into());
        } else {
            self.error_at_current("Unsupported Operand type.");
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
