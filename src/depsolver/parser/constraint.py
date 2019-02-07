from parser.version import parse_version
from model.constraint import VersionConstraint, Relation

def parse_constraints(json_list):
    return list(map(parse_constraint, json_list))

def parse_constraint(string):
    i = 0
    while i < len(string):
        if string[i:].startswith('<='):
            package_name = string[:i]
            version = parse_version(string[i + 2:])
            return VersionConstraint(package_name,
                                     Relation.less_than_or_equal,
                                     version)
        elif string[i:].startswith('>='):
            package_name = string[:i]
            version = parse_version(string[i + 2:])
            return VersionConstraint(package_name,
                                     Relation.greater_than_or_equal,
                                     version)
        elif string[i:].startswith('='):
            package_name = string[:i]
            version = parse_version(string[i + 1:])
            return VersionConstraint(package_name,
                                     Relation.equal,
                                     version)
        elif string[i:].startswith('>'):
            package_name = string[:i]
            version = parse_version(string[i + 1:])
            return VersionConstraint(package_name,
                                     Relation.greater_than,
                                     version)
        elif string[i:].startswith('<'):
            package_name = string[:i]
            version = parse_version(string[i + 1:])
            return VersionConstraint(package_name,
                                     Relation.less_than,
                                     version)
        else:
            i += 1
    return VersionConstraint(string, None, None)
