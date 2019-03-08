use crate::depsolver::model::constraint::PackageConstraint;
use crate::depsolver::model::command::Command;
use crate::depsolver::model::package::Package;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn shrink_problem<'a>(package_list: Vec<Package>,
                      mut initial_state_set: HashSet<usize>,
                      mut final_state: Vec<Command>)
    -> (Vec<Package>, HashSet<usize>, Vec<Command>) {
    let final_state_set : HashSet<usize> = {
        HashSet::from_iter(final_state.iter().map(|c| match c {
            Command::Install(
                PackageConstraint::Expanded { possibilities }) => possibilities.iter().cloned(),
            Command::Uninstall(
                PackageConstraint::Expanded { possibilities }) => possibilities.iter().cloned(),
            _ => panic!("Attempted to shrink before expansion.")
        }).flatten())
    };

    let (mut new_package_list, id_map) =
        depth_first_references(&package_list, final_state_set.union(&initial_state_set).cloned());

    fix_package_references(&mut new_package_list, &id_map);
    fix_package_references_in_initial_state(&mut initial_state_set, &id_map);
    fix_package_references_in_final_state(&mut final_state, &id_map);

    (new_package_list, initial_state_set, final_state)
}

fn depth_first_references<'a, I>(package_list: &Vec<Package>, roots: I)
     -> (Vec<Package>, HashMap<usize, usize>)
    where I: Iterator<Item=usize> {

    let mut visited_id_map: HashMap<usize, usize> = HashMap::new();
    let mut visited_list: Vec<Package> = vec![];
    let mut unvisited: Vec<usize> = Vec::from_iter(roots);

    while let Some(id) = unvisited.pop() {
        if !visited_id_map.contains_key(&id) {
            let package = &package_list[id];
            visited_id_map.insert(id, visited_list.len());

            for disjunction in &package.dependencies {
                for constraint in disjunction {
                    unvisited.extend(constraint.possibilities());
                }
            }

            for constraint in &package.conflicts {
                unvisited.extend(constraint.possibilities());
            }

            visited_list.push(package.clone());
        }
    }

    (visited_list, visited_id_map)
}

fn fix_package_references_in_final_state(final_state: &mut Vec<Command>, id_map: &HashMap<usize, usize>) {
    fn fix_references_in_command(command: &mut Command, id_map: &HashMap<usize, usize>) {
        match command {
            Command::Install(constraint) | Command::Uninstall(constraint) =>
                for package_id in constraint.possibilities_mut() {
                    *package_id = id_map[package_id]
                }
        }
    }

    for command in final_state {
        fix_references_in_command(command, id_map)
    }
}

fn fix_package_references_in_initial_state(initial_state: &mut HashSet<usize>, id_map: &HashMap<usize, usize>) {
    *initial_state = initial_state.iter().map(|package_id| id_map[package_id]).collect()
}

fn fix_package_references<'a>(packages: &mut Vec<Package>, id_map: &HashMap<usize, usize>) {
    fn fix_references_in_package(package: &mut Package, id_map: &HashMap<usize, usize>) {
        package.id = id_map[&package.id];
        for dependency in &mut package.dependencies {
            for alternative in dependency {
                for package_reference in alternative.possibilities_mut() {
                    *package_reference = id_map[&package_reference]
                }
            }
        }

        for conflict in &mut package.conflicts {
            for package_reference in conflict.possibilities_mut() {
                *package_reference = id_map[&package_reference]
            }
        }
    }

    for package in packages {
        fix_references_in_package(package, id_map);
    }
}
