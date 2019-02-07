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
