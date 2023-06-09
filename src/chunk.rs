use crate::opcode::OpCode;
use crate::value::{Value, ValueArray};
use crate::vm::InterpretResult;

#[derive(Debug, Clone)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write_opcode(&mut self, code: OpCode, line: usize) {
        self.code.push(code.into());
        self.lines.push(line);
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn read(&self, ip: usize) -> u8 {
        self.code[ip]
    }

    pub fn get_line(&self, ip: usize) -> usize {
        self.lines[ip]
    }

    pub fn free(&mut self) {
        self.code.clear();
        self.lines.clear();
        self.constants.free();
    }

    pub fn add_constant(&mut self, value: Value) -> Option<u8> {
        let idx = self.constants.write(value);
        match u8::try_from(idx) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn get_constant(&self, index: usize) -> Result<Value, InterpretResult> {
        self.constants.read_value(index)
    }

    pub fn disassemble<T: ToString>(&mut self, name: T) {
        println!("== {} ==", name.to_string());
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset)
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("    |   ");
        } else {
            print!("  {:4}  ", self.lines[offset]);
        }
        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::Return => self.simple_instruction("OP_RETURN", offset),
            OpCode::Constant => self.constant_instruction("OP_CONSTANT", offset),
            OpCode::Negate => self.simple_instruction("OP_NEGATE", offset),
            OpCode::Add => self.simple_instruction("OP_ADD", offset),
            OpCode::Subtract => self.simple_instruction("OP_SUBTRACT", offset),
            OpCode::Multiply => self.simple_instruction("OP_MULTIPLY", offset),
            OpCode::Divide => self.simple_instruction("OP_DIVIDE", offset),
            OpCode::Nil => self.simple_instruction("OP_NIL", offset),
            OpCode::True => self.simple_instruction("OP_TRUE", offset),
            OpCode::False => self.simple_instruction("OP_FALSE", offset),
            OpCode::Not => self.simple_instruction("OP_NOT", offset),
            OpCode::Equal => self.simple_instruction("OP_EQUAL", offset),
            OpCode::Greater => self.simple_instruction("OP_GREATER", offset),
            OpCode::Less => self.simple_instruction("OP_LESS", offset),
            OpCode::BangEqual => self.simple_instruction("OP_BANG_EQUAL", offset),
            OpCode::GreaterEqual => self.simple_instruction("OP_GREATER_EQUAL", offset),
            OpCode::LessEqual => self.simple_instruction("OP_LESS_EQUAL", offset),
            OpCode::Print => self.simple_instruction("OP_PRINT", offset),
            OpCode::Pop => self.simple_instruction("OP_POP", offset),
            OpCode::DefineGlobal => self.constant_instruction("OP_DEFINE_GLOBAL", offset),
            OpCode::GetGlobal => self.constant_instruction("OP_GET_GLOBAL", offset),
        }
    }

    pub fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    pub fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{name:-16} {constant:4} '");
        self.constants.print_value(constant as usize);
        println!("'");
        offset + 2
    }
}
