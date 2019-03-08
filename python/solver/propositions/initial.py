"""
At the beginning every package in the repository is either installed (True) or
uninstalled (False).
"""
from .utils import conjunction
from z3 import Not

def make_initial_state_constraint(package_list,
                                  initial_state,
                                  package_variables):
    def is_installed(id):
        return id in initial_state

    def install_state_constraint(package):
        if is_installed(package.id):
            return package_variables[package.id]
        else:
            return Not(package_variables[package.id])

    assert package_list, "No packages in the repository."
    return conjunction(map(install_state_constraint, package_list))
