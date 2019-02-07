"""
The number of changes between each state must be less than or equal to one.
This is defined using a Pseudo-Boolean less-than or equal constraint.
"""
from z3 import PbLe, Xor
from iterator_extensions import zip_with, adjacent_pairs
from .utils import conjunction

def make_one_change_at_a_time_invariant(package_variables):
    def one_at_a_time(prev_state, next_state):
        return PbLe(tuple(zip_with(lambda prev, next: (Xor(prev, next), 1),
                                   prev_state, next_state)), 1)
    return conjunction(adjacent_pairs(one_at_a_time, package_variables))
