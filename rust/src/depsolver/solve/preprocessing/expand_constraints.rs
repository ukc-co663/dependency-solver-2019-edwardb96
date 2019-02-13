use crate::depsolver::model::command::Command;
use crate::depsolver::model::constraint::PackageConstraint;
use crate::depsolver::model::package::{Package, PackageKey};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
//fn expand_constraints_in_problem(repo_package_list : Vec<Package>,
//                                 initial_state : Vec<PackageKey>,
//                                 final_state : Vec<PackageConstraint>)
//    ->  (Vec<Package>, Vec<PackageKey>, Vec<PackageConstraint>) {
//}

type NamePackageIndex<'a> = HashMap<String, Vec<&'a Package>>;

pub fn expand_constraints_in_problem(packages: Vec<Package>,
                                     initial: Vec<PackageKey>,
                                     final_state: Vec<Command>)
    -> (Vec<Package>, HashSet<i32>, Vec<Command>) {
    let index = name_to_packages_index(&packages);
    let expanded_package_list = expand_all_constraints(&packages, &index);
    let expanded_initial_state = expand_initial_state(&initial, &index);
    let expanded_final_state = expand_final_state(final_state, &index);
    (expanded_package_list,
     expanded_initial_state,
     expanded_final_state)
}

fn name_to_packages_index<'a>(package_list: &'a Vec<Package>)
    -> NamePackageIndex<'a> {
    let mut package_dictionary = NamePackageIndex::new();
    for package in package_list {
        package_dictionary.entry(package.name.clone())
                          .or_insert(vec![])
                          .push(package)
    }
    package_dictionary
}

fn expand_all_constraints(packages: &Vec<Package>, index: &NamePackageIndex) -> Vec<Package> {
    packages.into_iter()
            .map(|p| Package {
                id: p.id,
                name: p.name.clone(),
                version: p.version.clone(),
                size: p.size,
                dependencies:
                    p.dependencies.iter()
                                  .map(|dep| dep.iter()
                                                .map(|alt| expand_constraint(alt, index))
                                                .collect())
                                  .collect(),
                conflicts: p.conflicts.iter()
                                      .map(|con| expand_constraint(con, index))
                                      .collect()
            }).collect()
}

fn expand_initial_state<'a>(initial_state: &Vec<PackageKey>,
                            index: &NamePackageIndex<'a>)
    -> HashSet<i32> {
    fn expand_specific_package(index : &NamePackageIndex, package_key : &PackageKey) -> i32 {
        let PackageKey(name, version) = package_key;
        let packages_with_name = &index[name];
        packages_with_name.iter()
             .filter(|p| p.version == *version)
             .next()
             .expect("package in initial state does not exist in the repository")
             .id
    }
    return HashSet::from_iter(initial_state.iter()
                                           .map(|pk| expand_specific_package(index, pk)))
}

fn expand_final_state(final_state: Vec<Command>, index : &NamePackageIndex) -> Vec<Command> {
    fn expand_command(command: &Command, index: &NamePackageIndex) -> Command {
        match command {
            Command::Install(constraint) =>
                Command::Install(expand_constraint(&constraint, index)),
            Command::Uninstall(constraint) =>
                Command::Uninstall(expand_constraint(&constraint, index))
        }
    }

    final_state.iter()
               .map(|c| expand_command(c, index))
               .collect()
}

fn expand_constraint(constraint: &PackageConstraint, index: &NamePackageIndex)
    -> PackageConstraint {
    match &constraint {
        PackageConstraint::Unexpanded { name, version_constraint: _ } =>
            PackageConstraint::Expanded {
                possibilities: index[name].iter()
                                          .filter(|p| constraint.version_fulfils_constraint(&p.version))
                                          .map(|p| p.id)
                                          .collect()
            },
        _ => panic!("Attempted to expand constraint when already expanded.")
    }
}
