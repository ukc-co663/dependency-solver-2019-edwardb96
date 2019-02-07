from model.version import version_string

class Relation(object):
    less_than = 0
    greater_than = 1
    less_than_or_equal = 2
    greater_than_or_equal = 3
    equal = 4

def relation_string(self):
    if self == Relation.less_than:
        return "<"
    elif self == Relation.greater_than:
        return ">"
    elif self == Relation.less_than_or_equal:
        return "<="
    elif self == Relation.greater_than_or_equal:
        return ">="
    else:
        return "="

class VersionConstraint(object):
    def __init__(self, package_name, relation, version):
        self.package_name = package_name
        self.relation = relation
        self.version = version

    def is_name_only(self):
        return self.version is not None

    def as_version_predicate(self):
        if self.is_name_only():
            if Relation.equal == self.relation:
                return lambda pv: pv == self.version
            elif Relation.less_than == self.relation:
                return lambda pv: pv < self.version
            elif Relation.less_than_or_equal == self.relation:
                return lambda pv: pv <= self.version
            elif Relation.greater_than == self.relation:
                return lambda pv: pv > self.version
            elif Relation.greater_than_or_equal == self.relation:
                return lambda pv: pv >= self.version
        else:
            return lambda _: True

    def __str__(self):
        return self.package_name + \
            (relation_string(self.relation) + version_string(self.version)
             if self.is_name_only() else "")

    __repr__ = __str__
