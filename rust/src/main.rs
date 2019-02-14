extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::path::Path;
mod depsolver;

use depsolver::parse::parse_problem;
use depsolver::solve::solve;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let repo_path = Path::new(&args[1]);
    let initial_path = Path::new(&args[2]);
    let constraints_path = Path::new(&args[3]);

    let (repo, initial, final_state) =
        parse_problem(File::open(repo_path).expect("repository file does not exist"),
                      File::open(initial_path).expect("initial file does not exist"),
                      File::open(constraints_path).expect("constraints file does not exist"));
                      
    let commands = solve(repo, initial, final_state);

    println!("{:#?}", &commands);

}
