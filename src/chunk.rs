pub struct Chunk {
    code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, byte: OpCode) {
        self.code.push(byte.into());
    }

    pub fn free(&mut self) {
        self.code = Vec::new();
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

        let instruction: OpCode = self.code[offset].into();
        match instruction {
            OpCode::OpReturn => self.simple_instruction("OP_RETURN", offset),
        }
    }

    pub fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }
}

pub enum OpCode {
    OpReturn = 0,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::OpReturn,
            _ => unimplemented!("Invalid OpCode from u8"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        match code {
            OpCode::OpReturn => 0,
            // _ => unimplemented!("Invalid OpCode from opcode")
        }
    }
}
