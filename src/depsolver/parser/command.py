from model.command import Install, Uninstall
from parser.constraint import parse_constraint

def parse_command(string):
    if string[0] == '+':
        return Install(parse_constraint(string[1:]))
    elif string[0] == '-':
        return Uninstall(parse_constraint(string[1:]))
    else:
        assert False, "Bad commands"
