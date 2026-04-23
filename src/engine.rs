use std::{collections::HashMap, panic};
use crate::{formula::{BinaryOp, Formula, UnaryOp, Variable}, lexer::Lexer, parser::parse_formula};

#[derive(Clone, Debug)]
pub struct Node {
   pub id: usize,
   pub var: Option<String>,   // None = terminal
   pub left: Option<usize>,    // false branch
   pub right: Option<usize>,   // true branch
}

#[derive(Clone, Debug)]
pub enum ObddStep {
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
pub enum IntegrateStep {
    CheckEqual,
    LookupNode,
    CreateNode,
    Return,
}

#[derive(Clone, Debug)]
pub struct IntegrateFrame {
    pub step: IntegrateStep,
    pub n1: usize,
    pub p: String,
    pub n2: usize,
    pub result: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct ObddFrame {
    pub step: ObddStep,
    pub formula: Formula,
    pub p: Option<String>,
    pub var_index: usize,
    pub n1: Option<usize>,
    pub n2: Option<usize>,
    pub result: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum ExecutionFrame {
    Obdd(ObddFrame),
    Integrate(IntegrateFrame),
}

#[derive(Clone, Debug)]
pub struct OBDDEngineState {
    pub step_number: u32,
    pub nodes: Vec<Node>,
    unique_table: HashMap<(String, usize, usize), usize>,
    pub execution_stack: Vec<ExecutionFrame>,
    pub message: String,
}

pub struct OBDDEngine {
    input_formula: Formula,
    input_order: Vec<String>,
    curr_state: usize,
    state_history: Vec<OBDDEngineState>,
    final_result: Option<usize>,
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
            var_index: 0,
            n1: None,
            n2: None,
            result: None,
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
            final_result: None,
        })
    }

    pub fn get_state(&self) -> Result<OBDDEngineState, String> {
        self.state_history
            .get(self.curr_state)
            .cloned()
            .ok_or_else(|| "No state available".to_string())
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.curr_state + 1 < self.state_history.len() {
            self.curr_state += 1;
            return Ok(());
        }

        match self.gen_next_state() {
            Ok(next_state) => {
                self.curr_state += 1;
                self.state_history.push(next_state);
                Ok(())
            }
            Err(e) => {
                Err(format!("Failed to create new state: {}", e))
            }
        }
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
                ObddStep::Simplify      => self.simplify_step   (&mut next_state),
                ObddStep::CheckBottom   => self.check_bottom    (&mut next_state),
                ObddStep::CheckTop      => self.check_top       (&mut next_state),
                ObddStep::ChooseVariable=> self.choose_variable (&mut next_state),
                ObddStep::RecurseLow    => self.recurse_var     (&mut next_state, false),
                ObddStep::RecurseHigh   => self.recurse_var     (&mut next_state, true),
                ObddStep::CallIntegrate => self.call_integrate  (&mut next_state),
                ObddStep::Return        => self.obdd_return     (&mut next_state)
            },
            ExecutionFrame::Integrate(frame) => match frame.step {
                IntegrateStep::CheckEqual => self.check_equal(&mut next_state),
                IntegrateStep::LookupNode => self.lookup_node(&mut next_state),
                IntegrateStep::CreateNode => self.create_node(&mut next_state),
                IntegrateStep::Return     => self.integrate_return(&mut next_state),
            }
        }
        return Ok(next_state);
    }

    fn simplify_step    (&self,  next_state: &mut OBDDEngineState){
        // this step will find all instances of the current variable and preform simplification
        // rules based on the boolean expression

        let execution_stack =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut ObddFrame = match execution_stack {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        execution_frame.step = ObddStep::CheckBottom;
        let formula: &mut Formula = &mut execution_frame.formula;
        
        loop {
            let mut changes: u32 = 0;

            self.simplify_walk(formula, &mut changes);

            if changes == 0 {
                break;
            }
        };
    }



    fn simplify_walk(&self, formula: &mut Formula, changed: &mut u32) {
        match formula {
            Formula::Atom(_) => {}

            Formula::Unary { op, expr } => {
                match op {
                    UnaryOp::Not => {
                        match &mut **expr {
                            Formula::Atom(Variable::True) => {
                                *formula = Formula::Atom(Variable::False);
                                *changed += 1;
                            }
                            Formula::Atom(Variable::False) => {
                                *formula = Formula::Atom(Variable::True);
                                *changed += 1;
                            }
                            _ => {self.simplify_walk(expr, changed);}
                        }
                    }
                }
            }

            Formula::Binary { op, left: lhs, right: rhs } => {
                match op {
                    BinaryOp::Or => {
                        if matches!(**lhs, Formula::Atom(Variable::True))
                            || matches!(**rhs, Formula::Atom(Variable::True))
                        {
                            *formula = Formula::Atom(Variable::True);
                            *changed += 1;
                        } else if matches!(**lhs, Formula::Atom(Variable::False))
                            && matches!(**rhs, Formula::Atom(Variable::False)) {
                            *formula = Formula::Atom(Variable::False);
                            *changed += 1;
                        }else if matches!(**lhs, Formula::Atom(Variable::False)) {
                            *formula = (**rhs).clone();
                            *changed += 1;
                        }else if matches!(**rhs, Formula::Atom(Variable::False)) {
                            *formula = (**lhs).clone();
                            *changed += 1;
                        } else {
                            self.simplify_walk(lhs, changed);
                            self.simplify_walk(rhs, changed);
                        }

                    },
                    // A & T = A
                    // _ & F = F
                    // T & T = T
                    BinaryOp::And => {
                        if matches!(**lhs, Formula::Atom(Variable::False))
                            || matches!(**rhs, Formula::Atom(Variable::False))
                        {
                            *formula = Formula::Atom(Variable::False);
                            *changed += 1;
                        } else if matches!(**lhs, Formula::Atom(Variable::True))
                            && matches!(**rhs, Formula::Atom(Variable::True)) {
                            *formula = Formula::Atom(Variable::True);
                            *changed += 1;
                        }else if matches!(**lhs, Formula::Atom(Variable::True)) {
                            *formula = (**rhs).clone();
                            *changed += 1;
                        }else if matches!(**rhs, Formula::Atom(Variable::True)) {
                            *formula = (**lhs).clone();
                            *changed += 1;
                        } else {
                            self.simplify_walk(lhs, changed);
                            self.simplify_walk(rhs, changed);
                        }
                    },
                    BinaryOp::Iff => {
                        // F <=> F = T
                        // T <=> T = T
                        // F <=> T = F
                        // T <=> F = F
                        match (&**lhs, &**rhs) {
                            (Formula::Atom(Variable::True), Formula::Atom(Variable::True)) |
                            (Formula::Atom(Variable::False), Formula::Atom(Variable::False)) => {
                                *formula = Formula::Atom(Variable::True);
                                *changed += 1;
                            }

                            (Formula::Atom(Variable::True), Formula::Atom(Variable::False)) |
                            (Formula::Atom(Variable::False), Formula::Atom(Variable::True)) => {
                                *formula = Formula::Atom(Variable::False);
                                *changed += 1;
                            }
                            // A <=> T = A
                            // A <=> F = !A
                            _ => {
                                if matches!(**lhs, Formula::Atom(Variable::True)) {
                                    *formula = (**rhs).clone();
                                    *changed += 1;
                                } else if matches!(**rhs, Formula::Atom(Variable::True)) {
                                    *formula = (**lhs).clone();
                                    *changed += 1;
                                } else if matches!(**lhs, Formula::Atom(Variable::False)) {
                                    *formula = Formula::Unary {
                                        op: UnaryOp::Not,
                                        expr: rhs.clone(),
                                    };
                                    *changed += 1;
                                } else if matches!(**rhs, Formula::Atom(Variable::False)) {
                                    *formula = Formula::Unary {
                                        op: UnaryOp::Not,
                                        expr: lhs.clone(),
                                    };
                                    *changed += 1;
                                } else {
                                    self.simplify_walk(lhs, changed);
                                    self.simplify_walk(rhs, changed);
                                }
                            }
                        }
                    },
                    // A -> T = T
                    // F -> A = !A
                    BinaryOp::Implies => {
                        if matches!(**lhs, Formula::Atom(Variable::False)) 
                            || matches!(**rhs, Formula::Atom(Variable::True)) {
                            *formula = Formula::Atom(Variable::True);
                            *changed += 1;
                        }else if matches!(**lhs, Formula::Atom(Variable::True)) {
                            *formula = (**rhs).clone();
                            *changed += 1;
                        }else if matches!(**rhs, Formula::Atom(Variable::False)) {
                            *formula = Formula::Unary { 
                                op: UnaryOp::Not,
                                expr: lhs.clone(),
                            };
                            *changed += 1;
                        } else {
                            self.simplify_walk(lhs, changed);
                            self.simplify_walk(rhs, changed);
                        }
                    },
                }

            }
        }
    }

    fn substitute_bool_ast_walk(&self, formula: &mut Formula, curr_var: &String, var_value: bool){
        match formula {
            Formula::Atom(atom) => {
                match atom {
                    Variable::Variable(a) => {
                        if a.name == *curr_var {
                            if var_value {
                                *atom = Variable::True;
                            } else{
                                *atom = Variable::False;
                            }
                        }
                    },
                    Variable::True => {},
                    Variable::False => {},
                }
            },
            Formula::Unary { op: _, expr } => {self.substitute_bool_ast_walk(expr, curr_var, var_value);},
            Formula::Binary { op: _, left: lhs, right: rhs } =>  {
                self.substitute_bool_ast_walk(lhs, curr_var, var_value);
                self.substitute_bool_ast_walk(rhs, curr_var, var_value);
            },
            
        }
    }

    fn check_top        (&self, next_state: &mut OBDDEngineState){
        let execution_stack =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut ObddFrame = match execution_stack {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        let formula = &execution_frame.formula;
        if matches!(formula, Formula::Atom(Variable::True)){
            execution_frame.result = Some(1);
            execution_frame.step = ObddStep::Return;
        } else {
            execution_frame.step = ObddStep::ChooseVariable;
        }
        
    }
    
    fn check_bottom     (&self, next_state: &mut OBDDEngineState){
        let execution_stack =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut ObddFrame = match execution_stack {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        let formula = &execution_frame.formula;
        if matches!(formula, Formula::Atom(Variable::False)){
            execution_frame.result = Some(0);
            execution_frame.step = ObddStep::Return;
        } else {
            execution_frame.step = ObddStep::CheckTop;
        }
    }
    fn choose_variable(&self, next_state: &mut OBDDEngineState){
        let execution_stack =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut ObddFrame = match execution_stack {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        let var_option = match self.input_order.get(execution_frame.var_index) {
            Some(string) => {string},
            None => {panic!("This should never happen")},
        };
        
        execution_frame.p = Some(var_option.clone());
        execution_frame.step = ObddStep::RecurseLow;
    }

    fn recurse_var (&self, next_state: &mut OBDDEngineState, value: bool){
        let execution_stack_last =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut ObddFrame = match execution_stack_last {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        let var = match &execution_frame.p {
            Some(varname) => {varname},
            None => {panic!("This really should not happen")},
        };

        let mut formula = execution_frame.formula.clone();

        self.substitute_bool_ast_walk(&mut formula, var, value);
        // add a new execution_frame

        let new_frame = ExecutionFrame::Obdd(ObddFrame{
            step: ObddStep::Simplify,
            formula: formula.clone(),
            p: None,
            var_index: execution_frame.var_index+1,
            n1: None,
            n2: None,
            result: None,
        });
        let execution_stack: &mut Vec<ExecutionFrame> = next_state.execution_stack.as_mut();
        execution_stack.push(new_frame);
    }

    fn call_integrate   (&self, next_state: &mut OBDDEngineState){
        let execution_stack_last =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut ObddFrame = match execution_stack_last {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        let n1 = match execution_frame.n1 {
            Some(n1) => {n1},
            None => {panic!("N1 Should not be none")},
        };
        let n2 = match execution_frame.n2 {
            Some(n2) => {n2},
            None => {panic!("N2 Should not be none")},
        };

        let p = match &execution_frame.p {
            Some(p) => {p},
            None => {panic!("p Should not be none")},
        };

        let new_frame = ExecutionFrame::Integrate(IntegrateFrame { 
            step: IntegrateStep::CheckEqual, 
            n1: n1.clone(), 
            p: p.clone(), 
            n2: n2.clone(), 
            result: None,  
        });
        let execution_stack: &mut Vec<ExecutionFrame> = next_state.execution_stack.as_mut();
        execution_stack.push(new_frame);
    }

    fn obdd_return(&self, state: &mut OBDDEngineState) {
        let finished = state.execution_stack.pop().unwrap();

        let execution_frame: ObddFrame = match finished {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("This should never happen")
        };

        let child_result: usize = execution_frame.result.unwrap();

        // If no parent, we're done
        if let Some(parent) = state.execution_stack.last_mut() {

            let parent: &mut ObddFrame = match parent {
                ExecutionFrame::Obdd(frame) =>  frame,
                ExecutionFrame::Integrate(_) => panic!("This should never happen")
            };

            if parent.n1.is_none() {
                parent.n1 = Some(child_result);
                parent.step = ObddStep::RecurseHigh;
            } else {
                parent.n2 = Some(child_result);
                parent.step = ObddStep::CallIntegrate;
            }

        } else {
            // Final result of whole computation
        }
    }

    fn check_equal (&self, next_state: &mut OBDDEngineState){
        let execution_stack_last =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut IntegrateFrame = match execution_stack_last {
            ExecutionFrame::Obdd(_) =>  panic!("This should never happen"),
            ExecutionFrame::Integrate(frame) => frame,
        };

        if execution_frame.n1 == execution_frame.n2 {
            execution_frame.result = Some(execution_frame.n1.clone());
            execution_frame.step = IntegrateStep::Return;
        } else{
            execution_frame.step = IntegrateStep::LookupNode;
        }
    }

    fn lookup_node (&self, next_state: &mut OBDDEngineState){
        let execution_stack_last =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut IntegrateFrame = match execution_stack_last {
            ExecutionFrame::Obdd(_) =>  panic!("This should never happen"),
            ExecutionFrame::Integrate(frame) => frame,
        };

        let lookup_value = (execution_frame.p.clone(), execution_frame.n1, execution_frame.n2);
        

        match next_state.unique_table.get(&lookup_value) {
            Some(node)=> {
                execution_frame.step = IntegrateStep::Return;
                execution_frame.result = Some(node.clone());
            },
            None => {
                execution_frame.step = IntegrateStep::CreateNode;
            },
        }
    }
    
    fn create_node (&self, next_state: &mut OBDDEngineState){
        let execution_stack_last =  next_state.execution_stack.last_mut().unwrap();

        let execution_frame: &mut IntegrateFrame = match execution_stack_last {
            ExecutionFrame::Obdd(_) =>  panic!("This should never happen"),
            ExecutionFrame::Integrate(frame) => frame,
        };

        
        let new_index = next_state.nodes.len();

        next_state.nodes.push(Node{
            id: new_index,
            var: Some(execution_frame.p.clone()),
            left: Some(execution_frame.n1),
            right: Some(execution_frame.n2),
        });

        next_state.unique_table.insert(
            (execution_frame.p.clone(), execution_frame.n1, execution_frame.n2), 
            new_index
        );

        execution_frame.result = Some(new_index);
        execution_frame.step = IntegrateStep::Return;

    }
    
    fn integrate_return (&self, next_state: &mut OBDDEngineState){
        let finished = next_state.execution_stack.pop().unwrap();

        let execution_frame: IntegrateFrame = match finished {
            ExecutionFrame::Obdd(_) =>  panic!("This should never happen"),
            ExecutionFrame::Integrate(frame) => frame,
        };

        let child_result: usize = execution_frame.result.unwrap();

        let prev_frame = next_state.execution_stack.last_mut().unwrap();

        let obbd_execution_frame: &mut ObddFrame = match prev_frame {
            ExecutionFrame::Obdd(frame) =>  frame,
            ExecutionFrame::Integrate(_) => panic!("If this is not a obbdd frame then how did we get here?"),
        };
    
        obbd_execution_frame.result = Some(child_result);
        obbd_execution_frame.step = ObddStep::Return;
    }
}
