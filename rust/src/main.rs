extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate itertools;
extern crate z3;
extern crate chrono;

use std::fs::File;
use std::path::Path;
mod depsolver;

use depsolver::parse::parse_problem;
use depsolver::solver::solve;
use depsolver::serialize::serialize_commands;
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let repo_path = Path::new(&args[1]);
    let initial_path = Path::new(&args[2]);
    let constraints_path = Path::new(&args[3]);


    eprintln!("[{}] started parsing repository", Local::now().format("%H:%M:%S"));

    let (repo, initial, final_state) =
        parse_problem(File::open(repo_path).expect("repository file does not exist"),
                      File::open(initial_path).expect("initial file does not exist"),
                      File::open(constraints_path).expect("constraints file does not exist"));

    let commands = solve(repo, initial, final_state).expect("Problem unsatisfiable");
    serialize_commands(&commands);
}
