"""
Positive and negative constraints must be fulfilled by the final state.
"""
from z3 import Not
from solver.propositions.utils import disjunction, conjunction

def make_final_state_constraint(final_state, package_variables):
    def command_constraint(command):
        if command.is_install():
            return disjunction(map(lambda pid: package_variables[pid],
                                   command.constraint))
        else:
            return Not(disjunction(map(lambda pid: package_variables[pid],
                                       command.constraint)))

    return conjunction(map(command_constraint, final_state))
