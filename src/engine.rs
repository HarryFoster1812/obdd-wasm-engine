use wasm_bindgen::prelude::*;
use std::{collections::HashMap, ops::Deref};
use crate::{formula::Formula, lexer::Lexer, parser::parse_formula};

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    var: Option<String>,   // None = terminal
    left: Option<usize>,    // false branch
    right: Option<usize>,   // true branch
}

#[derive(Clone, Debug)]
enum ObddStep {
    Simplify,
    CheckBottom,
    CheckTop,
    ChooseVariable,
    RecurseLow,
    RecurseHigh,
    CallIntegrate,
    Return,
}

#[derive(Clone, Debug)]
enum IntegrateStep {
    CheckEqual,
    LookupNode,
    CreateNode,
    Return,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
struct IntegrateFrame {
    step: IntegrateStep,
    n1: usize,
    p: String,
    n2: usize,
    result: Option<usize>,
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
struct ObddFrame {
    step: ObddStep,
    formula: Formula,
    p: Option<String>,
    n1: Option<usize>,
    n2: Option<usize>,
}

#[derive(Clone, Debug)]
enum ExecutionFrame {
    Obdd(ObddFrame),
    Integrate(IntegrateFrame),
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct OBDDEngineState {
    step_number: u32,
    nodes: Vec<Node>,
    unique_table: HashMap<(String, usize, usize), usize>,
    execution_stack: Vec<ExecutionFrame>,
    message: String,
}

#[wasm_bindgen]
pub struct OBDDEngine {
    input_formula: Formula,
    input_order: Vec<String>,
    curr_state: usize,
    state_history: Vec<OBDDEngineState>,
}

#[wasm_bindgen]
impl OBDDEngine {
    pub fn new(input: &str, ordering: Vec<String>) -> Result<Self, String> {
        if input.is_empty() {
            return Err("Input should not be empty".into());
        }

        let mut lexer = Lexer::new(input)?;
        let ast = parse_formula(&mut lexer, 0)?;

        // create terminal nodes
        let false_node = Node {
            id: 0,
            var: None,
            left: None,
            right: None,
        };

        let true_node = Node {
            id: 1,
            var: None,
            left: None,
            right: None,
        };

        let mut nodes = Vec::new();
        nodes.push(false_node);
        nodes.push(true_node);

        // initial execution stack
        let initial_frame = ExecutionFrame::Obdd(ObddFrame {
            step: ObddStep::Simplify,
            formula: ast.clone(),
            p: None,
            n1: None,
            n2: None,
        });

        // initial state
        let initial_state = OBDDEngineState {
            step_number: 0,
            nodes,
            unique_table: HashMap::new(),
            execution_stack: vec![initial_frame],
            message: "Initialized engine with terminal nodes".into(),
        };

        Ok(Self {
            input_formula: ast,
            input_order: ordering,
            curr_state: 0,
            state_history: vec![initial_state],
        })
    }

    pub fn get_state(&self) -> Result<OBDDEngineState, String> {
        self.state_history
            .get(self.curr_state)
            .cloned()
            .ok_or_else(|| "No state available".to_string())
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.curr_state+1 < self.state_history.len() {
            self.curr_state += 1;
            return Ok(());

        }
        else{
            // create new state struct
            let next_state = self.gen_next_state();
            if next_state.is_ok(){
                self.curr_state+=1;
                self.state_history.push(next_state.unwrap());
                return Ok(());
            }
            
        }

        return Err("Failed to create new state".into());
    }

    pub fn step_back(&mut self) -> Result<(), String> {
        if self.curr_state == 0 {
            return Err("Already at initial state".into());
        }

        self.curr_state -= 1;
        Ok(())
    }
}

impl OBDDEngine {

    fn gen_next_state(&self) -> Result<OBDDEngineState, String>{
        let last_state = self.state_history.last().unwrap();
        let mut next_state = last_state.clone();
        next_state.step_number+=1;
        let execution_stack = last_state.execution_stack.last();
        if execution_stack.is_none(){

            return Err("execution_stack is empty".into());
        }

        match execution_stack.unwrap() {
            ExecutionFrame::Obdd(frame) => match frame.step {
                ObddStep::Simplify      => self.simplify_step   (last_state, &mut next_state),
                ObddStep::CheckBottom   => self.check_bottom    (last_state, &mut next_state),
                ObddStep::CheckTop      => self.check_top       (last_state, &mut next_state),
                ObddStep::ChooseVariable=> self.choose_variable (last_state, &mut next_state),
                ObddStep::RecurseLow    => self.recurse_low     (last_state, &mut next_state),
                ObddStep::RecurseHigh   => self.recurse_high    (last_state, &mut next_state),
                ObddStep::CallIntegrate => self.call_integrate  (last_state, &mut next_state),
                ObddStep::Return        => self.obbd_return     (last_state, &mut next_state)
            },
            ExecutionFrame::Integrate(frame) => match frame.step {
                IntegrateStep::CheckEqual => self.check_equal(last_state, &mut next_state),
                IntegrateStep::LookupNode => self.lookup_node(last_state, &mut next_state),
                IntegrateStep::CreateNode => self.create_node(last_state, &mut next_state),
                IntegrateStep::Return     => self.integrate_return(last_state, &mut next_state),
            }
        }
        return Ok(next_state);
    }

    fn simplify_step    (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){
        // this step will find all instances of the current variable and preform simplification
        // rules based on the boolean expression
    }
    fn check_top        (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn check_bottom     (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn choose_variable  (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn recurse_low      (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn recurse_high     (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn call_integrate   (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn obbd_return      (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}

    fn check_equal      (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn lookup_node      (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn create_node      (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}
    fn integrate_return (&self, last_state: &OBDDEngineState, next_state: &mut OBDDEngineState){}

}
