use wasm_bindgen::prelude::*;

pub mod formula;
pub mod lexer;
pub mod parser;
pub mod engine;

use crate::lexer::Lexer;
use crate::parser::parse_formula;
use crate::engine::OBDDEngine;
use crate::engine::ExecutionFrame;

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
pub struct JsEngineState {
    pub step_number: u32,
    pub nodes: Vec<JsNode>,
    pub execution_stack: Vec<JsExecutionFrame>,
    pub message: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum JsExecutionFrame {
    Obdd {
        step: String,
        formula: String,
        p: Option<String>,
        var_index: usize,
        n1: Option<usize>,
        n2: Option<usize>,
        result: Option<usize>,
    },
    Integrate {
        step: String,
        n1: usize,
        p: String,
        n2: usize,
        result: Option<usize>,
    }
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

        let stack = state.execution_stack.into_iter().map(|frame| {
            match frame {
                ExecutionFrame::Obdd(f) => JsExecutionFrame::Obdd {
                    step: format!("{:?}", f.step),
                    formula: f.formula.to_string(),
                    p: f.p,
                    var_index: f.var_index,
                    n1: f.n1,
                    n2: f.n2,
                    result: f.result,
                },
                ExecutionFrame::Integrate(f) => JsExecutionFrame::Integrate {
                    step: format!("{:?}", f.step),
                    n1: f.n1,
                    p: f.p,
                    n2: f.n2,
                    result: f.result,
                }
            }
        }).collect::<Vec<_>>();

        let js_state = JsEngineState {
            step_number: state.step_number,
            nodes: state.nodes.into_iter().map(|n| JsNode {
                id: n.id,
                var: n.var,
                left: n.left,
                right: n.right,
            }).collect(),
            execution_stack: stack,
            message: state.message,
        };

        serde_wasm_bindgen::to_value(&js_state)
            .map_err(|e| e.to_string())
    }
}
