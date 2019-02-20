mod preprocessing;
mod propositions;
mod postprocessing;

use crate::depsolver::model::command::Command;
use crate::depsolver::model::package::{Package, PackageKey};
use z3::{Config, Context, Optimize, CheckResult};

use self::preprocessing::expand_constraints::expand_constraints_in_problem;
use self::preprocessing::shrink_repository::shrink_problem;

use self::propositions::make_propositions_for_problem;
use self::propositions::cost::add_cost_constraint;
use self::postprocessing::extract_commands::extract_commands;

pub fn solve(repo: Vec<Package>,
             initial_state: Vec<PackageKey>,
             final_state: Vec<Command>) -> Option<Vec<Command>> {
    eprintln!("expanding constriants");
    let (expanded_repo, expanded_initial, expanded_final) =
        expand_constraints_in_problem(repo, initial_state, final_state);
    let size_before = expanded_repo.len();
    eprintln!("shrinking problem");
    let (shrunk_repo, shrunk_initial_state, shrunk_final) =
        shrink_problem(expanded_repo, expanded_initial, expanded_final);
    let size_after = shrunk_repo.len();

    let size_reduction_percent = ((size_before - size_after) as f64 / size_before as f64) * 100f64;

    eprintln!("shrunk problem from {} to {} ({:.2}%)", size_before, size_after, size_reduction_percent);
    let step_limit = std::cmp::max(shrunk_repo.len() * 2, 100);
    eprintln!("Config z3");
    let cfg = Config::new();
    eprintln!("Ctxt z3");
    let ctx = Context::new(&cfg);
    eprintln!("Opt z3");
    let opt = Optimize::new(&ctx);

    eprintln!("Timeout z3");
    opt.set_timeout(4 * 60 * 1000);

    eprintln!("begin making constraints");
    let (package_variables, all_constraints) = make_propositions_for_problem(
        &ctx, &shrunk_repo, shrunk_initial_state, shrunk_final, step_limit);
    eprintln!("end making constraints");

    all_constraints.map_or(Some(vec![]), |constraints| {
        eprintln!("sending constraints to z3");
        opt.assert(&constraints);
        eprintln!("adding cost optimization constraint");
        add_cost_constraint(&opt, &package_variables, &shrunk_repo);

        eprintln!("running smt solver");
        //println!("{}", opt);
        match opt.check_get_model() {
            CheckResult::Satisfiable(model) => {
                eprintln!("constructing solution from optimal satisfying assignment");
                let commands = extract_commands(&package_variables, &shrunk_repo, &model);
                Some(commands)
            },
            CheckResult::Unknown(model) => {
                eprintln!("constructing solution from sub-optimal satisfying assignment");
                let commands = extract_commands(&package_variables, &shrunk_repo, &model);
                Some(commands)
            },
            CheckResult::Unsatisfiable => None,
        }
    })
}
