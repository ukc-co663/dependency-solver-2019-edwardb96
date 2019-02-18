use crate::depsolver::model::command::Command;
use std::io::{stdout, Write};

pub fn serialize_commands(commands: &Vec<Command>) {
    serde_json::to_writer(stdout(), commands).expect("Error serializing command.");
    stdout().write(b"\n").expect("Failed to write a newline, how pathetic");
}
