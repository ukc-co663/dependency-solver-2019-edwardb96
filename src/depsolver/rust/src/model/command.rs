use std::fmt;
use super::constraint::PackageConstraint;

enum Command<'a> {
   Install(PackageConstraint<'a>),
   Uninstall(PackageConstraint<'a>)
}

impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Command::Install(c) =>
                write!(f, "+{}", c),
            Command::Uninstall(c) =>
                write!(f, "-{}", c)
        }
    }
}
