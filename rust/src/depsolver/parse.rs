use super::model::package::Package;
use super::model::command::Command;
use std::fs::File;

pub fn parse_repository(repository_file: File) -> Vec<Package> {
    serde_json::from_reader(repository_file).expect("error while reading json!")
}

pub fn parse_constraints(constraints_file: File) -> Vec<Command> {
    serde_json::from_reader(constraints_file).expect("error while reading json")
}

fn parse_repository_state() {

}
