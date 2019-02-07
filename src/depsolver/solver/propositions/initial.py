"""
At the beginning every package in the repository is either installed (True) or
uninstalled (False).
"""
from .utils import conjunction
from z3 import Not

def make_initial_state_constraint(package_list,
                                  initial_state,
                                  initial_package_var):
    def is_installed(name, version):
        return (name, version) in initial_state

    def install_state_constraint(package):
        if is_installed(package.name, package.version):
            return initial_package_var(package.id)
        else:
            return Not(initial_package_var(package.id))

    assert package_list, "No packages in the repository."
    return conjunction(map(install_state_constraint, package_list))
