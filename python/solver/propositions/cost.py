"""
The cost for a solution as the sum of individual transitions between states.
"""
from iterator_extensions import zip_with, adjacent_pairs
from functools import reduce, partial
from itertools import chain
from z3 import Sum, And, Not, If, Int
from sys import stderr

uninstall_cost = 1000000

def make_cost_constraint(package_variables, package_list):
    def state_transition_cost(prev_state, next_state):
        return Sum(list(zip_with(transition_cost,
                                 prev_state,
                                 next_state,
                                 map(lambda p: p.size, package_list))))

    transition_costs = \
        list(adjacent_pairs(state_transition_cost, package_variables))
    cost_variable = Int('cost')
    cost_constraint = (cost_variable == Sum(transition_costs))
    return cost_constraint, cost_variable

def transition_cost(prev, next, size):
    return If(And(Not(prev), next),
              size, If(And(prev, Not(next)), uninstall_cost, 0))
