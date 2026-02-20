use std::vec;

use wasm_bindgen::prelude::*;
pub mod formula;
pub mod lexer;
pub mod parser; 
pub mod engine;

use crate::lexer::Lexer;
use crate::parser::parse_formula;
use crate::engine::*;

#[wasm_bindgen]
pub fn validate_formula(input: &str) -> bool {
    let mut lexer = match Lexer::new(input) {
        Ok(l) => l,
        Err(_) => return false,
    };

    parse_formula(&mut lexer, 0).is_ok()
}

