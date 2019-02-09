from .propositions import make_propositions_for_problem
from .postprocessor import postprocess
from .preprocessing.expand_constraints import expand_constraints_in_problem
from .preprocessing.shrink_repository import shrink_problem

from z3 import Solver, Optimize, sat, unknown
from sys import stderr

def solve(repository_package_list, initial_state, final_state):
    expanded_package_list, expanded_initial_state, expanded_final_state = \
        expand_constraints_in_problem(repository_package_list,
                                      initial_state,
                                      final_state)

    shrunk_package_list, shrunk_initial_state, shrunk_final_state = \
        shrink_problem(expanded_package_list,
                       expanded_initial_state,
                       expanded_final_state)

    original_size = len(repository_package_list)
    new_size = len(shrunk_package_list)
    reduced_by = int(((original_size - new_size) / original_size) * 100)
    print("reduced size by {}%".format(reduced_by), file=stderr)

    step_limit = len(shrunk_package_list)
    print("making propositions", file=stderr)
    constraints, cost, package_variables = \
        make_propositions_for_problem(shrunk_package_list,
                                      shrunk_initial_state,
                                      shrunk_final_state,
                                      step_limit)

    #print(constraints.sexpr(), file=stderr)
    print("running sat solver...", file=stderr)
    opt = Solver()
    opt.add(constraints)
    #opt.minimize(cost)
    result = opt.check()
    if result == sat:
        print("postprocessing...")
        solution = opt.model()
        return postprocess(solution,
                           package_variables,
                           shrunk_package_list,
                           step_limit)
    elif result == unknown:
        print("Unknown - timeout?")
        return None
    else:
        return None
