from parser import parse_repository, parse_final_state_constraints, \
    parse_initial_state_constraints
from serializer import serialize_solution
from model.repository import repository_to_dictionary
from solver import solve
from argparse import ArgumentParser
from os.path import join, dirname
from sys import stderr

arg_parser = ArgumentParser(description='Process some integers.')
arg_parser.add_argument(
    'repository', type=str,
    help='The repository.json file containing all packages')
arg_parser.add_argument(
    'initial', type=str,
    help='The initial.json file containing the current system state')
arg_parser.add_argument(
    'constraints', type=str,
    help='The constraints.json file specifying the desired changes')
args = arg_parser.parse_args()
commands_file = join(dirname(args.initial), "commands.json")

print("parsing...", file=stderr)
package_list = parse_repository(args.repository)
initial_state = parse_initial_state_constraints(args.initial)
final_state = parse_final_state_constraints(args.constraints)
print("converting to dictionary...", file=stderr)
package_dict = repository_to_dictionary(package_list)

print("solving...", file=stderr)
solution = solve(package_list, package_dict, initial_state, final_state,
                 step_limit=len(package_list) * 2)
if solution:
    serialize_solution(solution)
else:
    print("No solution?", file=stderr)
