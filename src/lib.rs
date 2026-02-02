use wasm_bindgen::prelude::*;
pub mod formula;
pub mod lexer;
pub mod parser; 

#[wasm_bindgen]
pub fn validate_formula(formula: &str) -> bool {
    // check if the given boolean formula is syntactically correct
    false
}

fn parse_formula(tokens: Vec<lexer::Token>){

}


// fn simplify(formula: formula){}
