"""
All states are required to fulfil the dependency and conflict constraints.
"""
from .utils import disjunction, conjunction
from z3 import And, Not, Implies
from functools import partial

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
        alternative_as_list = list(matching(alternative))
        return disjunction(alternative_as_list) \
            if alternative_as_list else True


    #  TODO This is broken for empty list of alternatives.
    any_of_alternatives = list(map(alternative_constraint, dependency))
    return Implies(package_variable, disjunction(any_of_alternatives)) \
        if any_of_alternatives else True
