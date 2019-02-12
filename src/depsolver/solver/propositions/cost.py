"""
The cost for a solution as the sum of individual transitions between states.
"""
from iterator_extensions import zip_with, adjacent_pairs
from functools import reduce, partial
from itertools import chain
from z3 import Sum, And, Not

uninstall_cost = 1000000

def make_cost_constraint(opt, package_variables, package_list):
    def state_transition_cost(prev_state, next_state):
        return zip_with(partial(transition_cost, opt),
                        prev_state,
                        next_state,
                        map(lambda p: p.size, package_list))

    list(adjacent_pairs(state_transition_cost, package_variables))

def transition_cost(opt, prev, next, size):
    opt.add_soft(And(Not(prev), next), size)
    opt.add_sort(And(prev, Not(next)), uninstall_cost)
    return None
