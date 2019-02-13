extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::path::Path;
mod depsolver;

use depsolver::parse::{parse_repository, parse_constraints};

fn main() {

    let args = std::env::args().collect::<Vec<String>>();
    let repo_path = Path::new(&args[1]);
    let initial_path = Path::new(&args[2]);
    let constraints_path = Path::new(&args[3]);

    let repo = {
        let mut res = parse_repository(File::open(repo_path).expect("repository file does not exist"));
        for i in 0..res.len() {
            res[i].id = i as i32;
        }
        res
    };

    let constr = parse_constraints(File::open(constraints_path).expect("constraints file does not exist"));

    println!("{:#?}", repo);
    println!("{:#?}", constr);
}
