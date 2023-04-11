use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }
        let c = self.advance();

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::SemiColon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),
            '!' => {
                let equal = self.is_match('=');
                self.make_token(if equal {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let equal = self.is_match('=');
                self.make_token(if equal {
                    TokenType::Equal
                } else {
                    TokenType::Assign
                })
            }
            '<' => {
                let equal = self.is_match('=');
                self.make_token(if equal {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let equal = self.is_match('=');
                self.make_token(if equal {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => self.error_token("Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, ttype: TokenType) -> Token {
        Token::new(
            ttype,
            self.source[self.start..self.current].iter().collect(),
            self.line,
        )
    }

    fn error_token(&self, message: &str) -> Token {
        Token::new(TokenType::Error, message.to_string(), self.line)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if '/' == self.peek_next() {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => {
                    return;
                }
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_token("Unterminated string.")
        } else {
            self.advance();
            self.make_token(TokenType::String)
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphabetic() || self.peek() == '_' {
            self.advance();
        }
        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            'a' => self.check_keyword(1, "nd", TokenType::And),
            'c' => self.check_keyword(1, "lass", TokenType::Class),
            'e' => self.check_keyword(1, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'a' => self.check_keyword(2, "lse", TokenType::False),
                        'o' => self.check_keyword(2, "r", TokenType::For),
                        'u' => self.check_keyword(2, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => self.check_keyword(1, "f", TokenType::If),
            'n' => self.check_keyword(1, "il", TokenType::Nil),
            'o' => self.check_keyword(1, "r", TokenType::Or),
            'p' => self.check_keyword(1, "rint", TokenType::Print),
            'r' => self.check_keyword(1, "eturn", TokenType::Return),
            's' => self.check_keyword(1, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source[self.start + 1] {
                        'h' => self.check_keyword(2, "is", TokenType::This),
                        'r' => self.check_keyword(2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => self.check_keyword(1, "ar", TokenType::Var),
            'w' => self.check_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, ttype: TokenType) -> TokenType {
        // don`t need to check length, because we can check all characters.
        // if self.current - self.start != start + length {
        //     return TokenType::Identifier;
        // }
        let compare: String = self.source[self.start + start..self.current]
            .iter()
            .collect();
        if compare == rest {
            return ttype;
        }
        TokenType::Identifier
    }
}
