// this will parse a given formula into a ast
use crate::lexer::*;
use crate::formula::*;

pub fn parse_formula(lexer: &mut Lexer, min_bp: u8) -> Result<Formula, String> {
    let mut lhs: Formula = match lexer.next_token()? {
        Some(Token::Identifier(var)) =>  {
            if var.name == "T" { 
                Ok(Formula::Atom(Variable::True))
            } else if var.name == "F" {
                Ok(Formula::Atom(Variable::False))
            } else {
                Ok(Formula::Atom(Variable::Variable(var)))
            }
        }

        Some(Token::LeftParen) => {
            let expr = parse_formula(lexer, 0)?;
            match lexer.next_token()?{
                Some(Token::RightParen) => Ok(expr),
                t => Err(format!("Expected ')', got {:?}", t)),
            }
        }

        Some(tok) => {
            if let Some(r_bp) = prefix_binding_power(&tok) {
                let rhs = parse_formula(lexer, r_bp)?;
                Ok(Formula::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(rhs),
                })
            } else {
                Err(format!("Unexpected token: {:?}", tok))
            }
        }

        None => Err("Unexpected end of input".to_string()),
    }?;

    loop {
        let op = match lexer.peek_token()? {
            Some(tok) => tok,
            None => break,
        };


        let (l_bp, r_bp) = match infix_binding_power(&op) {
            Some(bp) => bp,
            None => break,
        };

        if l_bp < min_bp {
            break;
        }

        let _ = lexer.next_token()?;

        let rhs = parse_formula(lexer, r_bp)?;

        lhs = Formula::Binary {
            op: match op {
                Token::And => BinaryOp::And,
                Token::Or => BinaryOp::Or,
                Token::Implies => BinaryOp::Implies,
                Token::Iff => BinaryOp::Iff,
                _ => unreachable!(),
            },
            left: Box::new(lhs),
            right: Box::new(rhs),
        };
    }

    Ok(lhs)
}


fn infix_binding_power(token: &Token) -> Option<(u8, u8)> {
    match token {
        Token::And      => Some((7, 8)),
        Token::Or       => Some((5, 6)),
        Token::Implies  => Some((3, 4)), 
        Token::Iff      => Some((1, 2)),
        _ => None,
    }
}

fn prefix_binding_power(token: &Token) -> Option<u8> {
    match token {
        Token::Not => Some(9),
        _ => None,
    }
}




// TESTING


fn atom(name: &str) -> Formula {
    Formula::Atom(Variable::Variable(NamedVariable{ name: name.into() }))
}

fn not(expr: Formula) -> Formula {
    Formula::Unary {
        op: UnaryOp::Not,
        expr: Box::new(expr),
    }
}

fn bin(op: BinaryOp, l: Formula, r: Formula) -> Formula {
    Formula::Binary {
        op,
        left: Box::new(l),
        right: Box::new(r),
    }
}



#[cfg(test)]
mod tests {
    use super::*; // imports your lexer and Token

    #[test]
    fn parse_single_variable() {
        let mut lexer = Lexer::new("a").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        assert!(matches!(ast, Formula::Atom(_)));
    }


    #[test]
    fn parse_not() {
        let mut lexer = Lexer::new("~a").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = not(atom("a"));

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn parse_and() {
        let mut lexer = Lexer::new("a & b").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = bin(
            BinaryOp::And,
            atom("a"),
            atom("b"),
        );

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn precedence_and_over_or() {
        let mut lexer = Lexer::new("a & b | c").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = bin(
            BinaryOp::Or,
            bin(BinaryOp::And, atom("a"), atom("b")),
            atom("c"),
        );

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn not_binds_tighter_than_and() {
        let mut lexer = Lexer::new("~a & b").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = bin(
            BinaryOp::And,
            not(atom("a")),
            atom("b"),
        );

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn binary_ops_are_left_associative() {
        let mut lexer = Lexer::new("a -> b -> c").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = bin(
            BinaryOp::Implies,
            bin(BinaryOp::Implies, atom("a"), atom("b")),
            atom("c"),
        );

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn parentheses_override_precedence() {
        let mut lexer = Lexer::new("(a | b) & c").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = bin(
            BinaryOp::And,
            bin(BinaryOp::Or, atom("a"), atom("b")),
            atom("c"),
        );

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn complex_expression() {
        let mut lexer = Lexer::new("~a & b -> c | d <-> e").unwrap();
        let ast = parse_formula(&mut lexer, 0).unwrap();

        let expected = bin(
            BinaryOp::Iff,
            bin(
                BinaryOp::Implies,
                bin(BinaryOp::And, not(atom("a")), atom("b")),
                bin(BinaryOp::Or, atom("c"), atom("d")),
            ),
            atom("e")
        );

        assert_eq!(format!("{ast:?}"), format!("{expected:?}"));
    }


    #[test]
    fn plus_incorrectly_accpeted() {
        let mut lexer = Lexer::new("(x & z) + (y & !z)").unwrap();
        let ast = parse_formula(&mut lexer, 0);
        // This should not be correct, this should panic
        assert!(ast.is_err());
    }
}


