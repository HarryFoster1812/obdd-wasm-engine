use crate::formula::Variable;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    peeked: Option<char>,
    peeked_token: Option<Token>,
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Input should not be empty".into());
        }

        let mut chars = input.chars();
        let peeked = chars.next();

        Ok(Self {
            input,
            chars,
            peeked,
            peeked_token: None,
        })
    }

    fn peek(&self) -> Option<char> {
        self.peeked
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.peeked;
        self.peeked = self.chars.next();
        current
    }

    fn consume_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if !predicate(c) {
                break;
            }
            s.push(c);
            self.advance();
        }
        s
    }

    fn next_token_inner(&mut self) -> Result<Option<Token>, String> {
        // Skip whitespace
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.advance();
        }

        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        match c {
            '~' | '!' => {
                self.advance();
                Ok(Some(Token::Not))
            }
            '&' => {
                self.advance();
                Ok(Some(Token::And))
            }
            '|' => {
                self.advance();
                Ok(Some(Token::Or))
            }
            '(' => {
                self.advance();
                Ok(Some(Token::LeftParen))
            }
            ')' => {
                self.advance();
                Ok(Some(Token::RightParen))
            }

            '<' => {
                self.advance();
                if self.advance() == Some('-') && self.advance() == Some('>') {
                    Ok(Some(Token::Iff))
                } else {
                    Err("Expected <->".into())
                }
            }

            '-' => {
                self.advance();
                if self.advance() == Some('>') {
                    Ok(Some(Token::Implies))
                } else {
                    Err("Expected ->".into())
                }
            }

            _ => {
                // Identifier
                let name = self.consume_while(|c| {
                    !c.is_whitespace()
                        && !matches!(c, '!' | '~' | '&' | '|' | '-' | '<' | '(' | ')')
                });

                Ok(Some(Token::Identifier(Variable::new(name))))
            }
        }
    }
}


impl<'a> Lexer<'a> {
    pub fn peek_token(&mut self) -> Result<Option<Token>, String> {
        if self.peeked_token.is_none() {
            self.peeked_token = self.next_token_inner()?;
        }
        Ok(self.peeked_token.clone())
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, String> {
        if let Some(tok) = self.peeked_token.take() {
            return Ok(Some(tok));
        }
        self.next_token_inner()
    }
}



fn lex_string(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(input)?;
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token()? {
        tokens.push(token);
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
