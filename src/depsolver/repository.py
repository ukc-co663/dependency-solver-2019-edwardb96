def version_string(version):
    return '.'.join(map(str, version))

class Package(object):
    def __init__(self, id, name, version, size, dependencies, conflicts):
        assert (version is not None)
        self.id = id
        self.name = name
        self.version = version
        self.size = size
        self.dependencies = dependencies
        self.conflicts = conflicts

    def __str__(self):
        return "({} v{}, id: {}, depends: {}, conflicts: {})" \
            .format(self.name, version_string(self.version), self.id,
                    self.dependencies, self.conflicts)

    __repr__ = __str__


class Relation(object):
    less_than = 0
    greater_than = 1
    less_than_or_equal = 2
    greater_than_or_equal = 3
    equal = 4

def relation_string(self):
    if self == 0:
        return "<"
    elif self == 1:
        return ">"
    elif self == 2:
        return "<="
    elif self == 3:
        return ">="
    else:
        return "="

class VersionConstraint(object):
    def __init__(self, package_name, relation, version):
        self.package_name = package_name
        self.relation = relation
        self.version = version

    def has_version(self):
        return self.version is not None

    def __str__(self):
        return self.package_name + \
            (relation_string(self.relation) + version_string(self.version)
             if self.has_version() else "")

    __repr__ = __str__

class Install(object):
    def __init__(self, constraint):
        self.constraint = constraint

    def is_install(self):
        return True

    def __str__(self):
        return "+" + str(self.constraint)

    __repr__ = __str__

class Uninstall(object):
    def __init__(self, constraint):
        self.constraint = constraint

    def is_install(self):
        return False

    def __str__(self):
        return "-" + str(self.constraint)

    __repr__ = __str__

def repository_to_dictionary(package_list):
    package_dictionary = {}
    for package in package_list:
        if package.name in package_dictionary:
            package_dictionary[package.name].append(package)
        else:
            package_dictionary[package.name] = [package]
    return package_dictionary
