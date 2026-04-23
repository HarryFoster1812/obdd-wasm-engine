#[derive(Debug, Clone)]
pub struct NamedVariable {
    pub name: String
}

#[derive(Debug, Clone)]
pub enum Variable {
    True,
    False,
    Variable(NamedVariable),
}

impl NamedVariable {
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

impl Formula {
    pub fn to_string(&self) -> String {
        match self {
            Formula::Atom(var) => match var {
                Variable::True => "⊤".to_string(),
                Variable::False => "⊥".to_string(),
                Variable::Variable(v) => v.name.clone(),
            },

            Formula::Unary { op, expr } => {
                let inner = expr.to_string();
                match op {
                    UnaryOp::Not => format!("¬{}", inner),
                }
            }

            Formula::Binary { op, left, right } => {
                let l = left.to_string();
                let r = right.to_string();

                let op_str = match op {
                    BinaryOp::And => " ∧ ",
                    BinaryOp::Or => " ∨ ",
                    BinaryOp::Implies => " → ",
                    BinaryOp::Iff => " ↔ ",
                };

                format!("({}{}{})", l, op_str, r)
            }
        }
    }
}
