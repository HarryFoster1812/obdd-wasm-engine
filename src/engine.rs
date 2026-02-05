
struct Node {
id: usize,
    label: String,
}

struct Edge {
    from: usize,
    to: usize,
    value: bool,
}


struct OBDDEngineState {
    step_number: u32,
    current_variable_index: usize, // this is a key of input_order
    current_formula: Formula,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    active_node_id: usize, // key of nodes
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

        Ok(Self {
            input_formula: ast,
            input_order: ordering,
            curr_state: 0,
            state_history: Vec::new(),
        })
    }

    pub fn get_state() -> Result<OBDDEngineState, String> {
    }
}
