use crate::depsolver::model::package::Package;
use crate::depsolver::model::command::Command;
use z3::Ast;
use std::collections::HashSet;

mod utils;
use self::utils::ConditionIterator;
pub mod cost;
mod package_variables;
use self::package_variables::make_package_variables;
mod invariant;
use self::invariant::make_one_change_at_a_time_invariant;
mod validity;
use self::validity::make_validity_constraint;
mod initial;
use self::initial::make_initial_state_constraint;
mod final_state;
use self::final_state::make_final_state_constraint;


pub fn make_propositions_for_problem<'ctx>(ctx: &'ctx z3::Context,
                                           packages: &Vec<Package>,
                                           initial_state: HashSet<usize>,
                                           final_state: Vec<Command>,
                                           step_limit: usize)
    -> (Vec<Vec<Ast<'ctx>>>, Option<Ast<'ctx>>) {
    eprintln!("making package variables");
    let package_variables = make_package_variables(ctx, packages.len(), step_limit);
    eprintln!("making one change at a time constraint");
    let one_at_a_time = make_one_change_at_a_time_invariant(ctx, &package_variables);


    eprintln!("making validity constraint");
    let validity_constraint = make_validity_constraint(&packages, &package_variables);

    eprintln!("making initial constraint");
    let initial_constraint = make_initial_state_constraint(&initial_state,
                                                           &package_variables[0],
                                                           &packages);
    eprintln!("making final state constraint");
    let final_state_constraint = make_final_state_constraint(&final_state,
                                                             &package_variables[step_limit-1]);
    eprintln!("combining constraints");
    let combined_constraint = vec![
        one_at_a_time,
        validity_constraint,
        initial_constraint,
        final_state_constraint
    ].into_iter().filter_map(|c| c).conjunction();

    (package_variables, combined_constraint)
}
