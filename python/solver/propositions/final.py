"""
Positive and negative constraints must be fulfilled by the final state.
"""
from z3 import Not
from solver.propositions.utils import disjunction, conjunction

def make_final_state_constraint(final_state, matching):
    def command_constraint(command):
        if command.is_install():
            return disjunction(matching(command.constraint))
        else:
            return Not(disjunction(matching(command.constraint)))

    return conjunction(map(command_constraint, final_state))
