"""
For every package p, at every state, p it is either installed or
uninstalled, we need a boolean variable for each of these.
"""
from model.version import version_string
from z3 import Bool

def make_package_variables(package_list, step_limit):
    return [[make_variable_for_package(package, time)
             for package in package_list]
            for time in range(step_limit)]

def make_variable_for_package(package, time):
    def package_variable_name(package, time):
        return "package_{}_v{}_at_time_{}".format(
            package.name, version_string(package.version), time)
    return Bool(package_variable_name(package, time))

def variable_for_package(package_variables, at_time, package):
    return package_variables[at_time][package.id]
