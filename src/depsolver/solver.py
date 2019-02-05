from z3 import Bool, Implies, Or, Not, And
from repository import Relation, version_string
from functools import reduce, partial

def make_sat_problem(package_list, package_dict, final_state, step_limit):
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

def matching_packages(packages_dict, package_install_vars, time, constraint):
    packages_with_name = packages_dict[constraint.package_name]

    def map_to_var(list):
        return map(lambda p: package_install_vars[time][p.id], list)

    def filter_with_name(version_predicate):
        return filter(
            lambda p: version_predicate(p.version),
            packages_with_name)

    if constraint.relation is not None:
        if Relation.equal == constraint.relation:
            return map_to_var(filter_with_name(
                              lambda pv: pv == constraint.version))
        elif Relation.less_than == constraint.relation:
            return map_to_var(filter_with_name(
                              lambda pv: pv < constraint.version))
        elif Relation.less_than_or_equal == constraint.relation:
            return map_to_var(filter_with_name(
                              lambda pv: pv <= constraint.version))
        elif Relation.greater_than == constraint.relation:
            return map_to_var(filter_with_name(
                              lambda pv: pv > constraint.version))
        elif Relation.greater_than_or_equal == constraint.relation:
            return map_to_var(filter_with_name(
                              lambda pv: pv >= constraint.version))
    else:
        return map_to_var(packages_with_name)

def disjunction(args):
    return reduce(Or, args)

def conjunction(args):
    return reduce(And, args)
