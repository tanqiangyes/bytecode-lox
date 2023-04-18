pub enum OpCode {
    Constant = 0,
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Nil,
    True,
    False,
    Not,
    Equal,
    Greater,
    Less,
    BangEqual,
    GreaterEqual,
    LessEqual,
    Print,
    Pop,
    DefineGlobal,
    GetGlobal,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0 => OpCode::Constant,
            1 => OpCode::Return,
            2 => OpCode::Negate,
            3 => OpCode::Add,
            4 => OpCode::Subtract,
            5 => OpCode::Multiply,
            6 => OpCode::Divide,
            7 => OpCode::Nil,
            8 => OpCode::True,
            9 => OpCode::False,
            10 => OpCode::Not,
            11 => OpCode::Equal,
            12 => OpCode::Greater,
            13 => OpCode::Less,
            14 => OpCode::BangEqual,
            15 => OpCode::GreaterEqual,
            16 => OpCode::LessEqual,
            17 => OpCode::Print,
            18 => OpCode::Pop,
            19 => OpCode::DefineGlobal,
            20 => OpCode::GetGlobal,
            _ => unimplemented!("Invalid OpCode from u8"),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(code: OpCode) -> Self {
        match code {
            OpCode::Constant => 0,
            OpCode::Return => 1,
            OpCode::Negate => 2,
            OpCode::Add => 3,
            OpCode::Subtract => 4,
            OpCode::Multiply => 5,
            OpCode::Divide => 6,
            OpCode::Nil => 7,
            OpCode::True => 8,
            OpCode::False => 9,
            OpCode::Not => 10,
            OpCode::Equal => 11,
            OpCode::Greater => 12,
            OpCode::Less => 13,
            OpCode::BangEqual => 14,
            OpCode::GreaterEqual => 15,
            OpCode::LessEqual => 16,
            OpCode::Print => 17,
            OpCode::Pop => 18,
            OpCode::DefineGlobal => 19,
            OpCode::GetGlobal => 20,
            // _ => unimplemented!("Invalid OpCode from opcode")
        }
    }
}
