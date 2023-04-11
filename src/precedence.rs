#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Precedence {
    None = 0,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    pub fn next(&self) -> Self {
        use Precedence::*;
        match *self {
            None => Assignment,
            Assignment => Or,
            Or => And,
            And => Equality,
            Equality => Comparison,
            Comparison => Term,
            Term => Factor,
            Factor => Unary,
            Unary => Call,
            Call => Primary,
            Primary => None,
        }
    }

    pub fn previous(&self) -> Self {
        use Precedence::*;
        match *self {
            Assignment => None,
            Or => Assignment,
            And => Or,
            Equality => And,
            Comparison => Equality,
            Term => Comparison,
            Factor => Term,
            Unary => Factor,
            Call => Unary,
            Primary => Call,
            None => Primary,
        }
    }
}
