use crate::formula::Variable;

#[derive(Debug)]
pub enum Token {
    Identifier(Variable),
    Not,
    Implies,
    Iff,
    And,
    Or,
    LeftParen,
    RightParen,
}

pub fn lex_string(formula: &str) -> Result<Vec<Token>, String> {
    if formula.is_empty() {
        return Err(String::from("Input should not be empty"));
    }

    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = formula.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let curr = chars[index];

        // Skip whitespace
        if curr.is_whitespace() {
            index += 1;
            continue;
        }

        // Multi-character operators
        if curr == '<'  {
         if index + 2 >= chars.len() || chars[index + 1] != '-' || chars[index + 2] != '>' {
            return Err(format!("Failed to parse token at index: {}. Expected <->", index));
         } 
            tokens.push(Token::Iff);
            index += 3;
            continue;
        }

        if curr == '-'  {
            if index + 1 >= chars.len() || chars[index + 1] != '>' {
                return Err(format!("Failed to parse token at index: {}. Expected ->", index));
            }
            tokens.push(Token::Implies);
            index += 2;
            continue;
        }

        // Single-character operators
        match curr {
            '~' | '!' => {
                tokens.push(Token::Not);
                index += 1;
            }
            '&' => {
                tokens.push(Token::And);
                index += 1;
            }
            '|' => {
                tokens.push(Token::Or);
                index += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                index += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                index += 1;
            }
            _ => {
                // Start of a variable: collect until whitespace or operator
                let start = index;
                while index < chars.len()
                    && !chars[index].is_whitespace()
                    && !matches!(chars[index], '!' | '~' | '&' | '|' | '-' | '<' | '(' | ')')
                {
                    index += 1;
                }
                let var_name: String = chars[start..index].iter().collect();
                tokens.push(Token::Identifier(Variable::new(var_name)));
            }
        }
    }

    Ok(tokens)
}




#[cfg(test)]
mod tests {
    use super::*; // imports your lexer and Token

    #[test]
    fn test_empty_input() {
        assert!(lex_string("").is_err());
    }

    #[test]
    fn test_single_variable() {
        let tokens = lex_string("x").unwrap();
        assert_eq!(tokens.len(), 1);
        // check the first token is an Identifier
        assert!(matches!(tokens[0], Token::Identifier(_)));
        // compare token content depending on your Variable type
    
        match &tokens[0] {
            Token::Identifier(var) => {
                assert_eq!(var.name, "x");
            }
            _ => panic!("Expected Identifier"),
        }
    }

    #[test]
    fn test_multi_character_variable() {
        let tokens = lex_string("foo").unwrap();
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Identifier(_)));
        // compare token content depending on your Variable type
    
        match &tokens[0] {
            Token::Identifier(var) => {
                assert_eq!(var.name, "foo");
            }
            _ => panic!("Expected Identifier"),
        }
    }

    #[test]
    fn test_operators() {
        let tokens = lex_string("~x & y | z").unwrap();
        // check that tokens match [Not, Identifier(x), And, Identifier(y), Or, Identifier(z)]
        assert_eq!(tokens.len(), 6);
        assert!(matches!(tokens[0], Token::Not));
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert!(matches!(tokens[2], Token::And));
        assert!(matches!(tokens[3], Token::Identifier(_)));
        assert!(matches!(tokens[4], Token::Or));
        assert!(matches!(tokens[5], Token::Identifier(_)));
    }

    #[test]
    fn test_implication_and_iff() {
        let tokens = lex_string("a -> b <-> c").unwrap();
        // check that tokens match [Identifier(a), Implies, Identifier(b), Iff, Identifier(c)]
        assert!(matches!(tokens[0], Token::Identifier(_)));
        assert!(matches!(tokens[1], Token::Implies));
        assert!(matches!(tokens[2], Token::Identifier(_)));
        assert!(matches!(tokens[3], Token::Iff));
        assert!(matches!(tokens[4], Token::Identifier(_)));
    }

    #[test]
    fn test_parentheses() {
        let tokens = lex_string("(x & y)").unwrap();
        // check that tokens match [LeftParen, Identifier(x), And, Identifier(y), RightParen]
        assert!(matches!(tokens[0], Token::LeftParen));
        assert!(matches!(tokens[1], Token::Identifier(_)));
        assert!(matches!(tokens[2], Token::And));
        assert!(matches!(tokens[3], Token::Identifier(_)));
        assert!(matches!(tokens[4], Token::RightParen));
    }
}
