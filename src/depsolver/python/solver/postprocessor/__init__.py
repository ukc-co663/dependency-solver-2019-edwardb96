from iterator_extensions import zip_with, first_or_none, adjacent_pairs
from model.command import Install, Uninstall
from model.constraint import VersionConstraint, Relation

def postprocess(model, package_variables, package_list, step_limit):
    states = [[bool(model[package_variables[time][package_id]])
               for package_id in range(len(package_list))]
              for time in range(step_limit)]

    def make_command_if_difference(before_state, after_state):
        package_id = first_difference_index(before_state, after_state)
        if package_id is None:
            return None
        else:
            package = package_list[package_id]
            if after_state[package_id]:
                return Install(VersionConstraint(package.name,
                                                 Relation.equal,
                                                 package.version))
            else:
                return Uninstall(VersionConstraint(package.name,
                                                   Relation.equal,
                                                   package.version))

    optional_commands = adjacent_pairs(make_command_if_difference, states)
    commands = filter(lambda x: x is not None, optional_commands)
    return list(commands)

def first_difference_index(left_list, right_list):
    def index_if_different(left, right, index):
        return index if left != right else None

    assert len(left_list) == len(right_list)
    return first_or_none(filter(lambda index: index is not None,
                         zip_with(index_if_different,
                                  left_list,
                                  right_list,
                                  range(len(left_list)))))
