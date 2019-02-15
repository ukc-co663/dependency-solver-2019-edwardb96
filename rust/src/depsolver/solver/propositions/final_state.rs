use crate::depsolver::model::command::Command;
use super::utils::ConditionIterator;
use z3::Ast;

pub fn make_final_state_constraint<'ctx>(final_state: &Vec<Command>,
                                         final_state_variables: &Vec<Ast<'ctx>>)
    -> Option<Ast<'ctx>> {
    final_state.iter().filter_map(|command| match command {
        Command::Install(constraint) =>
            constraint.possibilities()
                      .iter()
                      .map(|&package_id| final_state_variables[package_id].clone())
                      .disjunction(),
        Command::Uninstall(constraint) =>
            constraint.possibilities()
                      .iter()
                      .map(|&package_id| final_state_variables[package_id].not())
                      .conjunction()
    }).conjunction()
}
