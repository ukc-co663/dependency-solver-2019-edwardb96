extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::path::Path;
mod depsolver;

use depsolver::parse::{parse_repository, parse_constraints, parse_initial_state};

fn main() {

    let args = std::env::args().collect::<Vec<String>>();
    let repo_path = Path::new(&args[1]);
    let initial_path = Path::new(&args[2]);
    let constraints_path = Path::new(&args[3]);

    let repo = parse_repository(File::open(repo_path)
                   .expect("repository file does not exist"));
    let constraint = parse_constraints(File::open(constraints_path)
                         .expect("constraints file does not exist"));
    let initial = parse_initial_state(File::open(initial_path)
                      .expect("initial file does not exist"));

    println!("{:#?}", &initial);
}
