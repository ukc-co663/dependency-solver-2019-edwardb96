from functools import reduce, partial
from itertools import chain

def shrink_problem(repo_package_list, initial_state_set, final_state):
    final_state_packages_set = set(reduce(chain,
                                          map(lambda c: c.constraint,
                                              final_state)))

    maybe_used_package_list, id_map = \
        depth_first_references(repo_package_list,
                               initial_state_set | final_state_packages_set)
    return (fix_all_references(maybe_used_package_list, id_map),
            list(fix_references(id_map, initial_state_set)),
            list(map(partial(fix_references_in_command, id_map),
                     final_state)))

def depth_first_references(package_list, roots):
    visited_id_map = {}
    visited_list = []
    unvisited = list(roots)

    while unvisited:
        id = unvisited.pop()
        if id not in visited_id_map:
            package = package_list[id]
            visited_id_map[id] = len(visited_list)
            visited_list.append(package)

            for disjunction in package.dependencies:
                for constraint in disjunction:
                    unvisited.extend(constraint)

            for constraint in package.conflicts:
                unvisited.extend(constraint)

    return visited_list, visited_id_map

def fix_all_references(package_list, id_map):
    def fix_references_in_package(package):
        package.id = id_map[package.id]
        package.dependencies = \
            list(map(lambda disjunction: list(
                     map(lambda constraint: list(fix_references(id_map,
                                                                constraint)),
                         disjunction)),
                 package.dependencies))

        package.conflicts = \
            list(map(lambda conflict: list(fix_references(id_map, conflict)),
                     package.conflicts))
        return package

    return list(map(fix_references_in_package, package_list))

def fix_references_in_command(id_map, command):
    command.constraint = list(fix_references(id_map, command.constraint))
    return command

def fix_references(id_map, references):
    return map(lambda id: id_map[id], references)
