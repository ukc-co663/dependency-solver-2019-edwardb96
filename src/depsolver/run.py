from parse import parse_repository, parse_final_state_constraints, \
    parse_initial_state_constraints
from repository import repository_to_dictionary
from solver import make_sat_problem

package_list = parse_repository("tests/example-0/repository.json")
package_dict = repository_to_dictionary(package_list)
final_state = parse_final_state_constraints("tests/example-0/constraints.json")
initial_state = parse_initial_state_constraints("tests/example-0/initial.json")

step_limit = 4
assert step_limit >= 2
sat_problem = make_sat_problem(package_list,
                               package_dict,
                               initial_state,
                               final_state,
                               step_limit)

print(sat_problem)


# x = Int('x')
# y = Int('y')
# solve(x > 2, y < 10, x + 2*y == 7)
