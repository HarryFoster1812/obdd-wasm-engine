use std::collections::HashMap;
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

#[derive(Clone, Debug)]
struct IntegrateFrame {
    step: IntegrateStep,
    n1: usize,
    p: String,
    n2: usize,
    result: Option<usize>,
}

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

#[derive(Clone, Debug)]
pub struct OBDDEngineState {
    step_number: u32,
    current_variable_index: usize, // this is a key of input_order
    nodes: Vec<Node>,
    unique_table: HashMap<(String, usize, usize), usize>,
    execution_stack: Vec<ExecutionFrame>,
    message: String,
}

struct OBDDEngine {
    input_formula: Formula,
    input_order: Vec<String>,
    curr_state: usize,
    state_history: Vec<OBDDEngineState>,
}

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
            current_variable_index: 0,
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
            
        }
        // else
        // create new state struct
        // run one step of the algorithm (take old state)
        // if state_history.len() != 0
        //      inc curr_state
        // add new state to the history
        // 
        Ok(())
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

    fn gen_state(&mut self, old_state:OBDDEngineState) -> Result<OBDDEngineState, String>{
        Ok(OBDDEngineState {
            step_number: 0,
            current_variable_index: 0, 
            nodes: Vec::new(),
            unique_table: HashMap::new(),
            execution_stack: vec![],
            message: "".into(), 
        })
    }
    // Simplify step
    // CheckBottom
    // CheckTop
    // ChooseVariable
    // RecurseLo
    // RecurseHigh
    // CallIntegrat,
    //Return
}
