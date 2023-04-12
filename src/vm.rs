use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::opcode::OpCode;
use crate::value::Value;

pub struct VM<'a> {
    chunk: &'a mut Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretResult> {
        let mut compiler = Compiler::new(self.chunk);
        compiler.compile(source)?;
        self.ip = 0;
        self.run()
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
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                }
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                OpCode::Negate => {
                    if self.peek(0).is_number() {
                        let value = self.pop();
                        self.stack.push(-value);
                    } else {
                        return self.runtime_error("Operand must be a number");
                    }
                }
                OpCode::Add => self.binary_op(|a, b| a + b)?,
                OpCode::Subtract => self.binary_op(|a, b| a - b)?,
                OpCode::Multiply => self.binary_op(|a, b| a * b)?,
                OpCode::Divide => self.binary_op(|a, b| a / b)?,
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::Not => {
                    let value = self.pop();
                    self.stack.push(Value::Boolean(value.is_falsy()))
                }
            }
        }
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack.len() - distance - 1]
    }

    fn read_byte(&mut self) -> OpCode {
        let val = self.chunk.read(self.ip).into();
        self.ip += 1;
        val
    }

    fn read_constant(&mut self) -> Value {
        let index = self.chunk.read(self.ip) as usize;
        self.ip += 1;
        self.chunk.get_constant(index)
    }

    fn binary_op(&mut self, op: fn(a: Value, b: Value) -> Value) -> Result<(), InterpretResult> {
        if !self.peek(0).is_number() || !self.peek(1).is_number() {
            return self.runtime_error("Operands must be numbers.");
        }
        let b = self.pop();
        let a = self.pop();
        self.stack.push(op(a, b));
        Ok(())
    }

    fn runtime_error<T: ToString + ?Sized>(&mut self, msg: &T) -> Result<(), InterpretResult> {
        let line = self.chunk.get_line(self.ip - 1);
        eprintln!("{}", msg.to_string());
        eprintln!("[line {line}] in script.");
        self.reset_stack();
        Err(InterpretResult::RuntimeError)
    }
}

pub enum InterpretResult {
    CompileError,
    RuntimeError,
}
