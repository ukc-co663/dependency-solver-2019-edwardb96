use super::model::package::Package;
use super::model::package::PackageKey;
use super::model::command::Command;
use std::fs::File;

pub fn parse_problem(repo: File, initial: File, constraints: File)
    -> (Vec<Package>, Vec<PackageKey>, Vec<Command>) {
    (parse_repository(repo), parse_initial_state(initial), parse_constraints(constraints))
}

fn parse_repository(repository_file: File) -> Vec<Package> {
    let mut repo: Vec<Package> = serde_json::from_reader(repository_file)
                                            .expect("error while reading repository.json");
    for i in 0..repo.len() {
        repo[i].id = i;
    }
    repo
}

fn parse_constraints(constraints_file: File) -> Vec<Command> {
    serde_json::from_reader(constraints_file).expect("error while reading constraints.json")
}

fn parse_initial_state(initial_state_file: File) -> Vec<PackageKey> {
    serde_json::from_reader(initial_state_file).expect("error while reading initial.json")
}
