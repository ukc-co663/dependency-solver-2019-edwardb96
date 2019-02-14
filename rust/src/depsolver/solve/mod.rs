pub mod preprocessing;
use crate::depsolver::model::command::Command;
use crate::depsolver::model::package::{Package, PackageKey};
use preprocessing::expand_constraints::expand_constraints_in_problem;
use preprocessing::shrink_repository::shrink_problem;

pub fn solve(repo: Vec<Package>,
             initial_state: Vec<PackageKey>,
             final_state: Vec<Command>) -> Option<Vec<Command>> {
    let (expanded_repo, expanded_initial, expanded_final) =
        expand_constraints_in_problem(repo, initial_state, final_state);
    let size_before = expanded_repo.len();
    let (shrunk_repo, shrunk_initial_state, shrunk_final) =
        shrink_problem(expanded_repo, expanded_initial, expanded_final);
    let size_after = shrunk_repo.len();

    let size_reduction_percent = ((size_before - size_after) as f64 / size_before as f64) * 100f64;

    println!("Shrunk problem from {} to {} ({:.2}%)", size_before, size_after, size_reduction_percent);
    //println!("{:#?}", &shrunk_repo);
    //println!("{:#?}", &shrunk_initial_state);
    //println!("{:#?}", &shrunk_final);

    return None
}
