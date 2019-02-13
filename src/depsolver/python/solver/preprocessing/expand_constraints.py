from functools import reduce
from iterator_extensions import first_or_none
from model.package import Package
from model.command import Install, Uninstall
from itertools import chain

def expand_constraints_in_problem(repo_package_list,
                                  initial_state,
                                  final_state):
    repo_package_dict = repository_to_dictionary(repo_package_list)
    expanded_initial_state = expand_initial_state(repo_package_dict,
                                                  initial_state)
    expanded_final_state = expand_final_state(repo_package_dict, final_state)
    expanded_repo_package_list = expand_all_constraints(repo_package_list,
                                                        repo_package_dict)
    return (expanded_repo_package_list,
            expanded_initial_state, expanded_final_state)

def repository_to_dictionary(package_list):
    package_dictionary = {}
    for package in package_list:
        if package.name in package_dictionary:
            package_dictionary[package.name].append(package)
        else:
            package_dictionary[package.name] = [package]
    return package_dictionary

def expand_all_constraints(repo_package_list, repo_package_dict):
    def expand_package_constraints(package):
        expanded_deps = list(map(lambda disjunction: list(
                                 map(lambda constraint: list(
                                     expand_constraint(
                                         repo_package_dict, constraint)),
                                     disjunction)),
                             package.dependencies))
        expanded_conflicts = list(map(lambda conflict: list(
                                      expand_constraint(
                                          repo_package_dict, conflict)),
                                      package.conflicts))
        return Package(package.id,
                       package.name,
                       package.version,
                       package.size,
                       expanded_deps,
                       expanded_conflicts)

    return list(map(expand_package_constraints, repo_package_list))

def expand_final_state(package_dict, final_state):
    def expand_command(command):
        constraint = list(expand_constraint(package_dict, command.constraint))
        if command.is_install():
            return Install(constraint)
        else:
            return Uninstall(constraint)

    return list(map(expand_command, final_state))

def expand_initial_state(repo_package_dict, initial_state):
    def expand_specific_package(name_version_pair):
        (name, version) = name_version_pair
        packages_with_name = repo_package_dict[name]
        return next(filter_by_version(lambda v: v == version,
                                      packages_with_name)).id
    return set(map(expand_specific_package, initial_state))


def expand_constraint(repo_packages_dict, constraint):
    if constraint.package_name in repo_packages_dict:
        packages_with_name = repo_packages_dict[constraint.package_name]
        return map(lambda p: p.id,
                   filter_by_version(constraint.as_version_predicate(),
                                     packages_with_name))
    else:
        return []

def filter_by_version(version_predicate, packages_with_name):
    return filter(lambda p: version_predicate(p.version),
                  packages_with_name)
