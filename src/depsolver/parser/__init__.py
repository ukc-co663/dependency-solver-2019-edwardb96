from .package import parse_package, parse_installed_package
from .command import parse_command
from iterator_extensions import zip_with
from json import load

def read_json_file(filename):
    with open(filename, 'r') as file:
        return load(file)

def parse_repository(filename):
    json_list = read_json_file(filename)
    return list(zip_with(parse_package, json_list, range(len(json_list))))

def parse_final_state_constraints(filename):
    json_list = read_json_file(filename)
    return list(map(parse_command, json_list))

def parse_initial_state_constraints(filename):
    json_list = read_json_file(filename)
    return set(map(parse_installed_package, json_list))
