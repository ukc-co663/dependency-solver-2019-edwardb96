"""
The cost for a solution as the sum of individual transitions between states.
"""
from iterator_extensions import zip_with, adjacent_pairs
from functools import reduce
from itertools import chain
from z3 import Int, If, Sum

uninstall_cost = 1000000

def make_cost_constraint(package_variables, package_list):
    def state_transition_cost(prev_state, next_state):
        return zip_with(transition_cost,
                        prev_state,
                        next_state,
                        map(lambda p: p.size, package_list))

    transition_costs = adjacent_pairs(state_transition_cost, package_variables)
    cost = Int('cost')
    constraint = (cost == Sum(*reduce(chain, transition_costs)))
    return constraint, cost

def transition_cost(prev, next, size):
    return If(prev,
              If(next, 0, uninstall_cost),
              If(next, size, 0))
