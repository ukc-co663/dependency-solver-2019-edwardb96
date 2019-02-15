use z3::Ast;
use super::utils::ConditionIterator;

pub fn make_one_change_at_a_time_invariant<'ctx>(package_variables: &Vec<Vec<Ast<'ctx>>>)
    -> Option<Ast<'ctx>> {

    fn one_at_a_time<'ctx>(prev_state: &Vec<Ast<'ctx>>, next_state: &Vec<Ast<'ctx>>) -> Option<Ast<'ctx>> {
        assert_eq!(prev_state.len(), next_state.len());
        let xors: Vec<Ast> = izip!(prev_state, next_state).map(|(prev, next)| prev.xor(next)).collect();
        let coefficients = vec![1; prev_state.len()];
        let mut xor_refs: Vec<&Ast> = xors.iter().collect();
        xor_refs.pop().map(|xor| xor.pb_le(&xor_refs[..], coefficients, 1))
    }

    package_variables.windows(2)
                     .filter_map(|w| match w {
                         [prev_state, next_state] => one_at_a_time(&prev_state, &next_state),
                         _ => panic!("window of wrong size")
                     })
                     .conjunction()

}
