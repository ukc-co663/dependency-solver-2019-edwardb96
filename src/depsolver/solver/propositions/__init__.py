from functools import partial
from .utils import conjunction
from .validity import make_validity_constraint
from .initial import make_initial_state_constraint
from .final import make_final_state_constraint
from .cost import make_cost_constraint
from .invariant import make_one_change_at_a_time_invariant
from .package_variables import make_package_variables, \
    variable_for_package

def make_propositions_for_problem(package_list, package_dict,
                                  initial_state, final_state, step_limit):
    assert package_list

    package_variables = make_package_variables(package_list, step_limit)
    cost_constraint, cost_objective = \
        make_cost_constraint(package_variables, package_list)
    constraints = [
        cost_constraint,
        make_one_change_at_a_time_invariant(package_variables),
        make_validity_constraint(
            package_list,
            package_variables,
            step_limit,
            partial(matching_package_variables,
                    package_dict,
                    package_variables)),
        make_initial_state_constraint(
            package_list,
            initial_state,
            lambda id: package_variables[0][id]),
        make_final_state_constraint(
            final_state,
            partial(matching_package_variables,
                    package_dict,
                    package_variables,
                    step_limit - 1)),
    ]
    return (conjunction(constraints), cost_objective, package_variables)

def filter_by_version(version_predicate, packages_with_name):
    return filter(lambda p: version_predicate(p.version),
                  packages_with_name)

def matching_package_variables(packages_dict, package_variables,
                               time, constraint):
    packages_with_name = packages_dict[constraint.package_name]
    version_predicate = constraint.as_version_predicate()
    return map(partial(variable_for_package, package_variables, time),
               filter_by_version(version_predicate, packages_with_name))
