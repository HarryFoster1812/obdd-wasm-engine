use wasm_bindgen::prelude::*;

pub mod formula;
pub mod lexer;
pub mod parser;
pub mod engine;

use crate::lexer::Lexer;
use crate::parser::parse_formula;
use crate::engine::OBDDEngine;

use serde::Serialize;

// Formula validation
#[wasm_bindgen]
pub fn validate_formula(input: &str) -> bool {
    let mut lexer = match Lexer::new(input) {
        Ok(l) => l,
        Err(_) => return false,
    };

    parse_formula(&mut lexer, 0).is_ok()
}

#[derive(Serialize)]
struct JsNode {
    id: usize,
    var: Option<String>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Serialize)]
struct JsState {
    step_number: u32,
    nodes: Vec<JsNode>,
    message: String,
}

// WASM wrapper for engine
#[wasm_bindgen]
pub struct WasmOBDDEngine {
    inner: OBDDEngine,
}

#[wasm_bindgen]
impl WasmOBDDEngine {

    #[wasm_bindgen(constructor)]
    pub fn new(input: &str, ordering: Vec<String>) -> Result<WasmOBDDEngine, String> {
        let engine = OBDDEngine::new(input, ordering)?;
        Ok(WasmOBDDEngine { inner: engine })
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.inner.step()
    }

    pub fn step_back(&mut self) -> Result<(), String> {
        self.inner.step_back()
    }

    pub fn get_state(&self) -> Result<JsValue, String> {
        let state = self.inner.get_state()?;

        let js_state = JsState {
            step_number: state.step_number,
            nodes: state.nodes.into_iter().map(|n| JsNode {
                id: n.id,
                var: n.var,
                left: n.left,
                right: n.right,
            }).collect(),
            message: state.message,
        };

        serde_wasm_bindgen::to_value(&js_state)
            .map_err(|e| e.to_string())
    }
}
