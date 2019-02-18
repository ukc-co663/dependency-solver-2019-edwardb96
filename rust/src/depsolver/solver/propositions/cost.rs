use crate::depsolver::model::package::Package;
use z3::{Optimize, Ast};

const UNINSTALL_COST : i64 = -1000000;

pub fn add_cost_constraint(opt : &Optimize,
                           package_variables: &Vec<Vec<Ast>>,
                           packages: &Vec<Package>) {

    fn add_state_transition_cost(opt: &Optimize,
                                 packages: &Vec<Package>,
                                 prev_state: &Vec<Ast>,
                                 next_state: &Vec<Ast>) {
        let sizes = packages.iter().map(|p| p.size);
        for (prev, next, size) in izip!(prev_state, next_state, sizes) {
            add_transition_cost(opt, prev, next, size)
        }
    }

    fn add_transition_cost(opt: &Optimize, prev: &Ast, next: &Ast, size: u32) {
        let weight : i64 = -(size as i64);
        opt.add_soft(&prev.not().and(&[next]), weight);
        opt.add_soft(&prev.and(&[&next.not()]), UNINSTALL_COST);
    }

    for w in package_variables.windows(2) {
        match w {
            [prev_state, next_state] =>
                add_state_transition_cost(opt, packages, &prev_state, &next_state),
            _ => panic!("matching window of wrong size")
        }
    }
}
