use crate::depsolver::model::package::Package;
use std::collections::HashSet;
use super::utils::ConditionIterator;
use z3::Ast;

pub fn make_initial_state_constraint<'ctx>(initial_state: &HashSet<usize>,
                                           initial_state_variables: &Vec<Ast<'ctx>>,
                                           packages: &Vec<Package>) -> Option<Ast<'ctx>> {
    (0..packages.len()).map(|package_id| {
        if initial_state.contains(&package_id) {
            initial_state_variables[package_id].clone()
        } else {
            initial_state_variables[package_id].not()
        }
    }).conjunction()
}
