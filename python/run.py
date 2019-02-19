from parser import parse_repository, parse_final_state_constraints, \
    parse_initial_state_constraints
from serializer import serialize_solution
from solver import solve
from sys import stderr, argv

# Move this to a new file and make it build a smaller list of packages.

_, repository_file, initial_file, constraints_file = argv

print("parsing...", file=stderr)
package_list = parse_repository(repository_file)
initial_state = parse_initial_state_constraints(initial_file)
final_state = parse_final_state_constraints(constraints_file)

print("solving...", file=stderr)
solution = solve(package_list, initial_state, final_state)
if solution is not None:
    serialize_solution(solution)
else:
    print("solver returned none")
