from json import loads
from repository import Relation, Package, VersionConstraint, Install, Uninstall
from zip_with import zip_with

def read_file(filename):
    with open(filename, 'r') as file:
        return file.read()

def read_json_file(filename):
    with open(filename, 'r') as file:
        return loads(file.read())

def parse_repository(filename):
    json_list = read_json_file(filename)
    return list(zip_with(parse_package, json_list, range(len(json_list))))

def parse_package(json_object, index):
    conflicts_constraints = parse_constraints(json_object["conflicts"]) \
        if "conflicts" in json_object else []
    depends_constraints = \
        list(map(parse_constraints, json_object["depends"])) \
        if "depends" in json_object else []

    return Package(id=index,
                   name=json_object["name"],
                   version=parse_version(json_object["version"]),
                   size=json_object["size"],
                   dependencies=depends_constraints,
                   conflicts=conflicts_constraints)

def parse_final_state_constraints(filename):
    json_list = read_json_file(filename)
    return list(map(parse_command, json_list))

def parse_command(string):
    if string[0] == '+':
        return Install(parse_constraint(string[1:]))
    elif string[0] == '-':
        return Uninstall(parse_constraint(string[1:]))
    else:
        assert False, "Bad commands"

def parse_version(version_string):
    return tuple(map(int, version_string.split('.')))

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
