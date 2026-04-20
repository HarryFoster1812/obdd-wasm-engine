use crate::OBDDEngine;

#[cfg(test)]
mod tests {
    #[test]
    fn trace_engine_states() {
        let input = "a & b | c";
        let ordering = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        let mut engine = OBDDEngine::new(input, ordering);

        for step in 0..200 {
            match engine.step() {
                Ok(_) => {
                    let state = engine.get_state();
                    println!("step {} -> state: {:?}", step, state);
                }
                Err(e) => {
                    println!("ERROR at step {}", step);
                    println!("reason: {}", e);

                    let state = engine.get_state();
                    println!("last state: {:?}", state);

                    panic!("stopped");
                }
            }
        }
    }
}
