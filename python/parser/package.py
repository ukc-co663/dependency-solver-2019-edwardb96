from parser.constraint import parse_constraints
from parser.version import parse_version
from model.package import Package

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

def parse_installed_package(string):
    package_name, version_string = string.split('=')
    return (package_name, parse_version(version_string))
