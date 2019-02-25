mod preprocessing;
mod propositions;
mod postprocessing;

use crate::depsolver::model::command::Command;
use crate::depsolver::model::package::{Package, PackageKey};
use z3::{Config, Context, Optimize, CheckResult};

use self::preprocessing::expand_constraints::expand_constraints_in_problem;
use self::preprocessing::shrink_repository::shrink_problem;

use self::propositions::make_propositions_for_problem;
use self::propositions::cost::make_cost_constraint;
use self::postprocessing::extract_commands::extract_commands;
use chrono::prelude::*;

pub fn solve(repo: Vec<Package>,
             initial_state: Vec<PackageKey>,
             final_state: Vec<Command>) -> Option<Vec<Command>> {
    eprintln!("[{}] preprocessing", Local::now().format("%H:%M:%S"));
    let (expanded_repo, expanded_initial, expanded_final) =
        expand_constraints_in_problem(repo, initial_state, final_state);
    let size_before = expanded_repo.len();
    eprintln!("shrinking problem");
    let (shrunk_repo, shrunk_initial_state, shrunk_final) =
        shrink_problem(expanded_repo, expanded_initial, expanded_final);
    let size_after = shrunk_repo.len();

    let size_reduction_percent = ((size_before - size_after) as f64 / size_before as f64) * 100f64;

    eprintln!("shrunk problem from {} to {} ({:.2}%)", size_before, size_after, size_reduction_percent);
    let step_limit = std::cmp::min(shrunk_repo.len() * 2, 200);
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let opt = Optimize::new(&ctx);

    opt.set_timeout(((4 * 60) + 30) * 1000);

    eprintln!("[{}] begin making constraints", Local::now().format("%H:%M:%S"));
    let (package_variables, all_constraints) = make_propositions_for_problem(
        &ctx, &shrunk_repo, shrunk_initial_state, shrunk_final, step_limit);

    all_constraints.map_or(Some(vec![]), |constraints| {
        eprintln!("sending problem constraints to z3");
        opt.assert(&constraints);
        eprintln!("adding cost optimization constraint");
        let (cost_variable, cost_constraint) =
            make_cost_constraint(&ctx, &package_variables, &shrunk_repo);
        opt.assert(&cost_constraint);
        eprintln!("[{}] end making constraints", Local::now().format("%H:%M:%S"));
        eprintln!("[{}] running smt solver", Local::now().format("%H:%M:%S"));
        opt.minimize(&cost_variable);
        //println!("{}", opt);
        match opt.check_get_model() {
            CheckResult::Satisfiable(model) => {
                eprintln!("[{}] constructing solution from optimal satisfying assignment",
                          Local::now().format("%H:%M:%S"));
                let commands = extract_commands(&package_variables, &shrunk_repo, &model);
                Some(commands)
            },
            CheckResult::Unknown(model) => {
                eprintln!("[{}] constructing solution from sub-optimal satisfying assignment",
                          Local::now().format("%H:%M:%S"));
                let commands = extract_commands(&package_variables, &shrunk_repo, &model);
                Some(commands)
            },
            CheckResult::Unsatisfiable => None,
        }
    })
}
