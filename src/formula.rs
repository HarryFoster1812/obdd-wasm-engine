#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String
}


impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    And,
    Or,
    Implies,
    Iff,
}

#[derive(Debug, Clone)]
pub enum Formula {
    Atom(Variable),
    Unary {
        op: UnaryOp,
        expr: Box<Formula>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Formula>,
        right: Box<Formula>,
    },
}
