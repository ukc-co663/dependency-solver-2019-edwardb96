"""
For every package p, at every state, p it is either installed or
uninstalled, we need a boolean variable for each of these.
"""
from model.version import version_string
from z3 import Bool
import sys

def make_package_variables(package_list, step_limit):
    lis = [[str(package.id) + " " + str(time)
            for package in package_list]
           for time in range(step_limit)]
    print(lis)
    sys.exit(-1)

def make_variable_for_package(package, time):
    def package_variable_name(package, time):
        return str(package.id) + "_" + str(time)
    return Bool(package_variable_name(package, time))

def variable_for_package(package_variables, at_time, package):
    return package_variables[at_time][package]
