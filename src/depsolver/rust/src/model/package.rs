use std::fmt;
use super::version::Version;
use super::constraint::PackageConstraint;

#[derive(Debug)]
struct Package<'a> {
    id : i32,
    name : &'a str,
    version : Version,
    size : i32,
    dependencies : Vec<Vec<PackageConstraint<'a>>>,
    conflicts : Vec<PackageConstraint<'a>>
}

impl<'a> fmt::Display for Package<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} v{}, id: {})",
               self.name, self.version, self.id)
    }
}
