from functools import partial
from .utils import conjunction
from .validity import make_validity_constraint
from .initial import make_initial_state_constraint
from .final import make_final_state_constraint
from .cost import make_cost_constraint
from .invariant import make_one_change_at_a_time_invariant
from .package_variables import make_package_variables, \
    variable_for_package
from sys import stderr

def make_propositions_for_problem(opt, package_list, initial_state,
                                  final_state, step_limit):
    assert package_list
    print("making package variables", file=stderr)
    package_variables = make_package_variables(package_list, step_limit)
    print("making cost constraint", file=stderr)
    make_cost_constraint(opt, package_variables, package_list)
    print("making one change constraint", file=stderr)
    one_change_constraint = \
        make_one_change_at_a_time_invariant(package_variables)
    print("making validity constraint", file=stderr)
    validity_constraint = \
        make_validity_constraint(
            package_list,
            package_variables,
            step_limit)
    print("making initial state constraint", file=stderr)
    initial_state_constraint = \
        make_initial_state_constraint(
            package_list,
            initial_state,
            package_variables[0])
    print("making final state constraint", file=stderr)
    final_state_constraint = \
        make_final_state_constraint(
            final_state,
            package_variables[step_limit - 1])

    constraints = [
        one_change_constraint,
        validity_constraint,
        initial_state_constraint,
        final_state_constraint
    ]
    return (conjunction(constraints), package_variables)
