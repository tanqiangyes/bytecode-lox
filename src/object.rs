use crate::object::Object::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Object {
    Str(String),
}

impl Object {}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Str(s) => Object::Str(s.clone()),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Str(v) => {
                write!(f, "{v}")
            }
        }
    }
}
