use crate::token_type::TokenType;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            ttype: TokenType::Undefined,
            lexeme: "".to_string(),
            line: 0,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            ttype: self.ttype,
            lexeme: self.lexeme.clone(),
            line: self.line,
        }
    }
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            line,
        }
    }
    pub fn is(&self, ttype: TokenType) -> bool {
        self.ttype == ttype
    }

    pub fn token_type(&self) -> TokenType {
        self.ttype
    }

    pub fn as_string(&self) -> String {
        self.lexeme.clone()
    }

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}  {}", self.line, self.ttype, self.lexeme)
    }
}
