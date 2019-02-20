from functools import reduce
from z3 import Or, And

def disjunction(args):
    cls = list(args)
    if len(cls) == 1:
        return cls[0]
    elif len(cls) > 0:
        return Or(cls)
    else:
        assert False, "You messed up"

def conjunction(args):
    cls = list(args)
    if len(cls) == 1:
        return cls[0]
    elif len(cls) > 0:
        return And(cls)
    else:
        assert False, "You messed up"
