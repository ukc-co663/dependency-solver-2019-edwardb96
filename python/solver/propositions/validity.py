"""
All states are required to fulfil the dependency and conflict constraints.
"""
from .utils import disjunction, conjunction
from z3 import And, Not, Implies
from functools import partial

def make_validity_constraint(package_list,
                             package_variables,
                             step_limit):
    validity_constraints = [
        package_validity_constraint(package,
                                    package_variables[time][package.id],
                                    package_variables[time])
        for package in package_list
        for time in range(step_limit)]
    return conjunction(validity_constraints)

def package_validity_constraint(package,
                                package_variable,
                                package_variables):
    make_dependency_constraint = \
        partial(dependency_constraint,
                package_variable,
                package_variables)
    make_conflict_constraint = \
        partial(conflict_constraint,
                package_variable,
                package_variables)

    all_dependencies_installed = \
        conjunction(map(make_dependency_constraint, package.dependencies)) \
        if package.dependencies else True

    no_conflicting_installed = \
        conjunction(map(make_conflict_constraint, package.conflicts)) \
        if package.conflicts else True

    return And(all_dependencies_installed, no_conflicting_installed)

def conflict_constraint(package_variable,
                        package_variables,
                        conflict):
    matches = list(map(lambda pid: package_variables[pid], conflict))
    return Not(And(package_variable, disjunction(matches))) \
        if matches else True

def dependency_constraint(package_variable,
                          package_variables,
                          dependency):
    def alternative_constraint(alternative):
        matches = list(map(lambda pid: package_variables[pid], alternative))
        return disjunction(matches) \
            if matches else True

    any_of_alternatives = list(map(alternative_constraint, dependency))
    return Implies(package_variable, disjunction(any_of_alternatives)) \
        if any_of_alternatives else True
