use crate::depsolver::model::command::Command;
use crate::depsolver::model::package::Package;
use crate::depsolver::model::constraint::{PackageConstraint, Relation};
use z3::{Model, Ast};

pub fn extract_commands(package_variables: &Vec<Vec<Ast>>,
                        packages: &Vec<Package>,
                        solution: &Model) -> Vec<Command> {
                            
    fn diff_states(solution: &Model,
                   packages: &Vec<Package>,
                   prev_state: &Vec<Ast>,
                   next_state: &Vec<Ast>) -> Option<Command> {
        assert!(!packages.is_empty());
        izip!(packages, prev_state, next_state).map(|(package, prev, next)| {
            let (prev_bool, next_bool) = (solution.eval(prev).unwrap().as_bool().unwrap(),
                                          solution.eval(next).unwrap().as_bool().unwrap());
            match (prev_bool, next_bool) {
                (true, true) | (false, false) => None,
                (true, false) => Some(Command::Uninstall(
                    PackageConstraint::Unexpanded {
                        name: package.name.clone(),
                        version_constraint: Some((Relation::Equal, package.version.clone())) } )),
                (false, true) => Some(Command::Install(
                    PackageConstraint::Unexpanded {
                        name: package.name.clone(),
                        version_constraint: Some((Relation::Equal, package.version.clone())) }))
            }
        }).flatten().next()
    }

    package_variables.windows(2)
                     .map(|s| match s {
                         [prev_state, next_state] =>
                            diff_states(solution, packages, &prev_state, &next_state),
                        _ => panic!("slices of wrong size")
                     })
                     .filter_map(|c| c)
                     .collect()
}
