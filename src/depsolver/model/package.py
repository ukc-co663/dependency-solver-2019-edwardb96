from model.version import version_string

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
