from functools import reduce
from z3 import Or, And

def disjunction(args):
    return reduce(Or, args)

def conjunction(args):
    return reduce(And, args)
