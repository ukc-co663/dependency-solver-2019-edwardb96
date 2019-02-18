use crate::depsolver::model::command::Command;
use crate::depsolver::model::package::{Package, PackageKey};
use z3::{Config, Context, Optimize};

mod preprocessing;
use preprocessing::expand_constraints::expand_constraints_in_problem;
use preprocessing::shrink_repository::shrink_problem;

mod propositions;
use propositions::make_propositions_for_problem;
use propositions::cost::add_cost_constraint;

mod postprocessing;
use postprocessing::extract_commands::extract_commands;

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
    //println!("{:#?}", &shrunk_repo);
    //println!("{:#?}", &shrunk_initial_state);
    //println!("{:#?}", &shrunk_final);
    let step_limit = shrunk_repo.len() * 2;
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    eprintln!("begin making constraints");
    let (package_variables, all_constraints) = make_propositions_for_problem(
        &ctx, &shrunk_repo, shrunk_initial_state, shrunk_final, step_limit);
    eprintln!("end making constraints");

    all_constraints.map_or(Some(vec![]), |constraints| {
        let opt = Optimize::new(&ctx);
        eprintln!("sending constraints to z3");
        opt.assert(&constraints);
        eprintln!("adding const optimization constraint");
        add_cost_constraint(&opt, &package_variables, &shrunk_repo);

        eprintln!("{}", &opt);
        
        eprintln!("running smt solver");
        if opt.check() {
            let model = opt.get_model();
            eprintln!("constructing solution from satisfying assignment");
            let commands = extract_commands(&package_variables, &shrunk_repo, &model);
            Some(commands)
        } else {
            eprintln!("Check returned false");
            None
        }
    })
}
