def Bool(name):
    return name

def Int(name):
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

def Xor(lhs, rhs):
    return "({} /= {})".format(lhs, rhs)

def If(if_, then_, else_):
    return "(if {} then {} else {})".format(if_, then_, else_)

def Sum(*args):
    return "({})".format(" + ".join(args))

def PbLe(tuple, n):
    def pretty_tuple(t):
        return ", ".join(map(lambda x: "{} costing {}".format(x[0], x[1]), t))
    return "(cost({}) <= {})".format(pretty_tuple(tuple), n)
