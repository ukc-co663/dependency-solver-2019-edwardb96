use crate::depsolver::model::constraint::PackageConstraint;
use crate::depsolver::model::package::Package;
use z3::{Ast, Context};
use super::utils::ConditionIterator;

pub fn make_validity_constraint<'ctx>(ctx: &'ctx Context,
                                      packages: &Vec<Package>,
                                      package_variables: &Vec<Vec<Ast<'ctx>>>) -> Option<Ast<'ctx>> {
    package_variables.iter()
                     .map(|state| {
                         packages.iter()
                                 .zip(state.iter())
                                 .filter_map(move |(package, package_var)| {
                                    package_validity_constraint(ctx, package, package_var, &state)
                                 })
                     })
                     .flatten()
                     .conjunction()
}

fn package_validity_constraint<'ctx>(ctx: &'ctx Context,
                                     package: &Package,
                                     package_variable: &Ast<'ctx>,
                                     state: &Vec<Ast<'ctx>>) -> Option<Ast<'ctx>> {

    let all_dependencies_installed: Option<Ast> =
        package.dependencies.iter()
                            .filter_map(|dep| {
                                if dep.iter().all(|alternative| alternative.possibilities().is_empty()) {
                                    Some(package_variable.implies(&ctx.from_bool(false)))
                                } else {
                                    dependency_constraint(package_variable, dep, state)
                                }
                            })
                            .conjunction();

    let no_conflicting_installed: Option<Ast> =
        package.conflicts.iter()
                         .filter_map(|conflict| conflict_constraint(package_variable, conflict, state))
                         .conjunction();

    match (all_dependencies_installed, no_conflicting_installed) {
        (Some(deps), Some(conf)) => Some(deps.and(&[&conf])),
        (Some(deps), None) => Some(deps),
        (None, Some(conf)) => Some(conf),
        (None, None) => None
    }
}

fn dependency_constraint<'ctx>(package_variable: &Ast<'ctx>,
                               alternatives: &Vec<PackageConstraint>,
                               state: &Vec<Ast<'ctx>>) -> Option<Ast<'ctx>> {
    let dependencies_installed = alternatives.iter()
                                             .map(|alternative| {
                                                 alternative.possibilities()
                                                            .iter()
                                                            .map(|&package_id| state[package_id].clone())
                                             })
                                             .flatten()
                                             .disjunction();
    dependencies_installed.map(|dep| package_variable.implies(&dep))
}

fn conflict_constraint<'ctx>(package_variable: &Ast<'ctx>,
                             constraint: &PackageConstraint,
                             state: &Vec<Ast<'ctx>>) -> Option<Ast<'ctx>> {
    let possibilities = constraint.possibilities();
    if !possibilities.is_empty() {
        let any_conflicts_installed =
            possibilities.iter()
                         .map(|&package_id| state[package_id].clone())
                         .disjunction();

        any_conflicts_installed.map(|conflicts_installed| {
            package_variable.and(&[&conflicts_installed]).not()
        })
    } else {
        None
    }
}
