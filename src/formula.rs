#[derive(Debug)]
pub struct Variable {
    pub name: String
}


impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct UnaryConnective {
    formula: Box<Formula>,
}

#[derive(Debug)]
pub struct BinaryConnective {
    left: Box<Formula>,
    right: Box<Formula>,
}

#[derive(Debug)]
pub enum Formula{
    VAR(Variable),
    OR(BinaryConnective),
    AND(BinaryConnective),
    NOT(UnaryConnective),
    IMPLIES(BinaryConnective),
    EQUIVALENCE(BinaryConnective)
}


