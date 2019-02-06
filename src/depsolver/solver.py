from z3 import Bool, Implies, Or, Not, And
from repository import Relation, version_string
from functools import reduce, partial

def make_sat_problem(package_list,
                     package_dict,
                     initial_state,
                     final_state,
                     step_limit):
    package_variables = [
        [var_for_package(package, time) for package in package_list]
        for time in range(step_limit)]

    expand_version_constraint = \
        partial(matching_packages, package_dict, package_variables)

    if package_list:
        validity_constraint = \
            make_validity_constraint(
                package_list,
                package_variables,
                step_limit,
                expand_version_constraint)

        initial_state_constraint = \
            make_initial_state_constraint(
                initial_state,
                partial(equal_package, package_dict, package_variables, 0))

        final_state_constraint = \
            make_final_state_constraint(
                final_state,
                partial(expand_version_constraint, step_limit - 1))

        return And(validity_constraint, final_state_constraint)
    else:
        return None

def make_final_state_constraint(final_state, matching):
    def command_constraint(command):
        if command.is_install():
            return disjunction(matching(command.constraint))
        else:
            return Not(disjunction(matching(command.constraint)))

    return conjunction(map(command_constraint, final_state))

def make_initial_state_constraint(initial_state, initial_package_var):
    def installed_package(initial_package):
        (package_name, version) = initial_package
        return initial_package_var(package_name, version)

    return conjunction(map(installed_package, initial_state)) \
        if initial_state else True

def make_validity_constraint(package_list,
                             package_variables,
                             step_limit,
                             expand_version_constraint):
    validity_constraints = [
        package_validity_constraint(package,
                                    package_variables[time][package.id],
                                    partial(expand_version_constraint, time))
        for package in package_list
        for time in range(step_limit)]
    return conjunction(validity_constraints)

def var_for_package(package, time):
    def package_var_name(package, time):
        return "{}_v{}_at_time_{}".format(
            package.name, version_string(package.version), time)
    return Bool(package_var_name(package, time))

def package_validity_constraint(package,
                                package_variable,
                                version_constraint_expander):
    make_dependency_constraint = \
        partial(dependency_constraint,
                package_variable,
                version_constraint_expander)
    make_conflict_constraint = \
        partial(conflict_constraint,
                package_variable,
                version_constraint_expander)

    all_dependencies_installed = \
        conjunction(map(make_dependency_constraint, package.dependencies)) \
        if package.dependencies else True

    no_conflicting_installed = \
        conjunction(map(make_conflict_constraint, package.conflicts)) \
        if package.conflicts else True

    return And(all_dependencies_installed, no_conflicting_installed)

def conflict_constraint(package_variable,
                        matching,
                        conflict):
    #  ¬(package_variable and (or... matching(conflict)))
    # TODO This is slightly broken for cases where constraint does not match
    # any package.
    matches = list(matching(conflict))
    return Not(And(package_variable, disjunction(matches))) \
        if matches else True

def dependency_constraint(package_variable,
                          matching,
                          dependency):
    def alternative_constraint(alternative):
        # TODO This is broken for cases where constraint does not match
        # any package. If this is the case we should probably error.
        return disjunction(matching(alternative))

    # TODO This is broken for empty list of alternatives.
    any_of_alternatives = map(alternative_constraint, dependency)
    return Implies(package_variable, disjunction(any_of_alternatives))

def filter_from_relation(relation, version):
    if Relation.equal == relation:
        return lambda pv: pv == version
    elif Relation.less_than == relation:
        return lambda pv: pv < version
    elif Relation.less_than_or_equal == relation:
        return lambda pv: pv <= version
    elif Relation.greater_than == relation:
        return lambda pv: pv > version
    elif Relation.greater_than_or_equal == relation:
        return lambda pv: pv >= version

def map_to_var(package_variables, time, list):
    return map(lambda p: package_variables[time][p.id], list)

def filter_by_version(version_predicate, packages_with_name):
    return filter(lambda p: version_predicate(p.version),
                  packages_with_name)

def equal_package(packages_dict,
                  package_variables,
                  time,
                  package_name,
                  package_version):
    packages_with_name = packages_dict[package_name]
    return next(map_to_var(package_variables, time,
                           filter_by_version(lambda pv: pv == package_version,
                                             packages_with_name)))

def matching_packages(packages_dict, package_variables, time, constraint):
    packages_with_name = packages_dict[constraint.package_name]
    if constraint.relation is not None:
        f = filter_from_relation(constraint.relation, constraint.version)
        return map_to_var(package_variables,
                          time,
                          filter_by_version(f, packages_with_name))
    else:
        return map_to_var(package_variables, time, packages_with_name)

def disjunction(args):
    return reduce(Or, args)

def conjunction(args):
    return reduce(And, args)
