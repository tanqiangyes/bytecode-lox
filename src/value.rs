use crate::object::Object;
use crate::vm::InterpretResult;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Nil,
    Obj(Object),
}

impl Value {
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::Obj(Object::Str(_)))
    }

    pub fn is_falsy(&self) -> bool {
        matches!(self, Value::Nil | Value::Boolean(false))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(v) => {
                write!(f, "{v}")
            }
            Value::Number(v) => {
                write!(f, "{v}")
            }
            Value::Nil => {
                write!(f, "nil")
            }
            Value::Obj(o) => {
                write!(f, "{}", o)
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
            _ => panic!("Only support numbers and numbers."),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left), Value::Number(right)) => {
                if right == 0 as f64 {
                    panic!("Can't divide by zero.")
                } else {
                    Value::Number(left / right)
                }
            }
            _ => panic!("Only support numbers and numbers."),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
            _ => panic!("Only support numbers and numbers."),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            (Value::Obj(Object::Str(left)), Value::Obj(Object::Str(right))) => {
                Value::Obj(Object::Str(format!("{}{}", left, right)))
            }
            (Value::Number(left), Value::Obj(Object::Str(right))) => {
                Value::Obj(Object::Str(format!("{}{}", left, right)))
            }
            (Value::Obj(Object::Str(left)), Value::Number(right)) => {
                Value::Obj(Object::Str(format!("{}{}", left, right)))
            }
            _ => panic!("Only support numbers and numbers."),
        }
    }
}

impl Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(v) => Value::Number(-v),
            _ => panic!("Only support numbers."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> usize {
        let count = self.values.len();
        self.values.push(value);
        count
    }

    pub fn free(&mut self) {
        self.values.clear();
    }

    pub fn print_value(&self, which: usize) {
        print!("{}", self.values[which]);
    }

    pub fn read_value(&self, which: usize) -> Result<Value, InterpretResult> {
        if let Some(value) = self.values.get(which) {
            Ok(value.clone())
        } else {
            Err(InterpretResult::CompileError)
        }
    }
}
