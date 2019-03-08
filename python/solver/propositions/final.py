"""
Positive and negative constraints must be fulfilled by the final state.
"""
from z3 import Not
from solver.propositions.utils import disjunction, conjunction

def make_final_state_constraint(final_state, package_variables):
    def command_constraint(command):
        if command.is_install():
            matching_packages = list(map(lambda pid: package_variables[pid],
                                     command.constraint))
            return disjunction(matching_packages) \
                if matching_packages else False
        else:
            matching_packages = list(map(lambda pid: package_variables[pid],
                                         command.constraint))
            return Not(disjunction(matching_packages)) \
                if matching_packages else True

    return conjunction(map(command_constraint, final_state))
