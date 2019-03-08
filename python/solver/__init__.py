from .propositions import make_propositions_for_problem
from .postprocessor import postprocess
from .preprocessing.expand_constraints import expand_constraints_in_problem
from .preprocessing.shrink_repository import shrink_problem

from z3 import Optimize, sat, unknown, Z3Exception
from sys import stderr

def solve(repository_package_list, initial_state, final_state):
    if final_state:
        if repository_package_list:
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

            opt = Optimize()
            step_limit = new_size * 2
            print("making propositions", file=stderr)
            constraints, package_variables, cost_variable = \
                make_propositions_for_problem(opt, shrunk_package_list,
                                              shrunk_initial_state,
                                              shrunk_final_state,
                                              step_limit)

            opt.set('timeout', ((60 * 5) + 30) * 1000)
            opt.add(constraints)
            opt.minimize(cost_variable)
            print("running sat solver...", file=stderr)
            result = opt.check()
            if result == sat:
                print("postprocessing...", file=stderr)
                solution = opt.model()
                return postprocess(solution,
                                   package_variables,
                                   shrunk_package_list,
                                   step_limit)
            elif result == unknown:
                try:
                    solution = opt.model()
                    print("sub optimal solution available", file=stderr)
                    return postprocess(solution,
                                       package_variables,
                                       shrunk_package_list,
                                       step_limit)
                except Z3Exception:
                    print("sub optimal solution unavailable aborting", file=stderr)
                    return None
            else:
                print("constraints unsatisfiable aborting", file=stderr)
                return None
        else:
            print("No packages in repository")
            return None
    else:
        return []
