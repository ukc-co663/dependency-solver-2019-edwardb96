def Bool(name):
    return name

def Implies(lhs, rhs):
    return "({} → {})".format(lhs, rhs)

def Or(lhs, rhs):
    if lhs is False:
        return str(rhs)
    elif rhs is False:
        return str(lhs)
    else:
        return "({} ∨ {})".format(lhs, rhs)

def Not(arg):
    return "(¬{})".format(arg)

def And(lhs, rhs):
    if lhs is True:
        return str(rhs)
    elif rhs is True:
        return str(lhs)
    else:
        return "({} ∧ {})".format(lhs, rhs)
