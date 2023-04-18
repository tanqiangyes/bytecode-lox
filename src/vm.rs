use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::object::Object;
use crate::opcode::OpCode;
use crate::value::Value;

pub struct VM<'a> {
    chunk: &'a mut Chunk,
    ip: usize,
    stack: Vec<Value>,
    memory: Vec<Object>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
            memory: Vec::new(),
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn free(&mut self) {
        self.stack.clear();
        self.chunk.free();
        self.ip = 0;
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretResult> {
        let mut compiler = Compiler::new(self.chunk);
        compiler.compile(source)?;
        self.ip = 0;
        let result = self.run();
        self.free();
        result
    }

    fn run(&mut self) -> Result<(), InterpretResult> {
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("           ");
                for slot in &self.stack {
                    print!("[ {slot} ]");
                }
                println!();
                let _ = &self.chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte();

            match instruction {
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::Print => {
                    println!("{}", self.stack.pop().unwrap());
                }
                OpCode::Return => {
                    return Ok(());
                }
                OpCode::Constant => {
                    let constant = self.read_constant()?;
                    self.stack.push(constant);
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::Equal => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::Boolean(a == b));
                }
                OpCode::BangEqual => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::Boolean(a != b));
                }
                OpCode::Greater => self.binary_op(|a, b| Value::Boolean(a > b))?,
                OpCode::GreaterEqual => self.binary_op(|a, b| Value::Boolean(a >= b))?,
                OpCode::Less => self.binary_op(|a, b| Value::Boolean(a < b))?,
                OpCode::LessEqual => self.binary_op(|a, b| Value::Boolean(a <= b))?,
                OpCode::Add => self.binary_op(|a, b| a + b)?,
                OpCode::Subtract => self.binary_op(|a, b| a - b)?,
                OpCode::Multiply => self.binary_op(|a, b| a * b)?,
                OpCode::Divide => self.binary_op(|a, b| a / b)?,
                OpCode::Not => {
                    let value = self.pop();
                    self.stack.push(Value::Boolean(value.is_falsy()))
                }
                OpCode::Negate => {
                    if self.peek(0).is_number() {
                        let value = self.pop();
                        self.stack.push(-value);
                    } else {
                        return self.runtime_error("Operand must be a number");
                    }
                }
            }
        }
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&mut self, distance: usize) -> Value {
        if let Some(value) = self.stack.get(self.stack.len() - distance - 1) {
            value.clone()
        } else {
            self.runtime_error("Get value failed.")
                .expect("Get value failed.");
            Value::Nil
        }
    }

    fn read_byte(&mut self) -> OpCode {
        let val = self.chunk.read(self.ip).into();
        self.ip += 1;
        val
    }

    fn read_constant(&mut self) -> Result<Value, InterpretResult> {
        let index = self.chunk.read(self.ip) as usize;
        self.ip += 1;
        self.chunk.get_constant(index)
    }

    fn binary_op(&mut self, op: fn(a: Value, b: Value) -> Value) -> Result<(), InterpretResult> {
        if (self.peek(0).is_number() && self.peek(1).is_number())
            || (self.peek(0).is_string() && self.peek(1).is_string())
            || (self.peek(0).is_number() && self.peek(1).is_string())
            || (self.peek(0).is_string() && self.peek(1).is_number())
        {
            let b = self.pop();
            let a = self.pop();
            self.stack.push(op(a, b));
            return Ok(());
        }
        self.runtime_error("Operands must be number or string.")
    }

    fn runtime_error<T: ToString + ?Sized>(&mut self, msg: &T) -> Result<(), InterpretResult> {
        let line = self.chunk.get_line(self.ip - 1);
        eprintln!("{}", msg.to_string());
        eprintln!("[line {line}] in script.");
        self.reset_stack();
        Err(InterpretResult::RuntimeError)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InterpretResult {
    CompileError,
    RuntimeError,
}
