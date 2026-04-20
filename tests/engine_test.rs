#[cfg(test)]
mod tests {
    use obdd::engine::OBDDEngine;

    #[test]
    fn trace_engine_states() {
        let input = "(a&c) | (b & !c)";
        let ordering = vec!["a".to_string(), "b".to_string(), "c".to_string()];

        let mut engine: OBDDEngine = OBDDEngine::new(input, ordering).unwrap();

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
