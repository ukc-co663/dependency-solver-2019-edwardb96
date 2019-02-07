from .propositions import make_propositions_for_problem
from .postprocessor import postprocess
from z3 import Optimize, sat, unknown
from sys import stderr

def solve(package_list, package_dict,
          initial_state, final_state, step_limit=6):
    assert step_limit >= 3, "step_limit must be greater than 3."

    print("making propositions", file=stderr)
    constraints, cost, package_variables = \
        make_propositions_for_problem(package_list, package_dict,
                                      initial_state, final_state,
                                      step_limit)

    print("running sat solver...", file=stderr)
    opt = Optimize()
    opt.add(constraints)
    opt.minimize(cost)
    result = opt.check()
    if result == sat:
        print("postprocessing...")
        solution = opt.model()
        return postprocess(solution, package_variables, package_list, step_limit)
    elif result == unknown:
        print("Unknown - timeout?")
        return None
    else:
        return None
